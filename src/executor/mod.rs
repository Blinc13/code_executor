use std::{
    str::FromStr,
    process::Stdio,
    path::{PathBuf, Path}
};
use tokio::{
    fs,
    time::Duration,
    process::Command,
    io::AsyncWriteExt
};
use serenity::model::id::{ChannelId, UserId};
use tracing::info;

mod utils;

#[derive(Copy, Clone, Debug)]
pub enum Language {
    Rust,
    Cpp,
    C
}

#[derive(Copy, Clone, Debug)]
pub enum Error {
    BuildError,
    ExecError,
    InvokeError,
    FsError,
    Unsupported,
    TimeOut
}

struct Id {
    channel: ChannelId,
    user: UserId
}

pub struct Executor {
    code: String,
    lang: Language,
    id: Id
}


// TODO: Move some code to modules
impl Executor {
    pub fn new(lang: Language, code: String, channel: ChannelId, user: UserId) -> Self {
        Self {
            code,
            lang,
            id: Id {
                channel,
                user
            }
        }
    }

    pub async fn compile_and_run(self, timeout: Duration) -> Result<(String, String), (Option<String>, Error)> {
        let code_path = PathBuf::from(Self::form_file_name(self.id.channel, self.id.user, self.lang.as_str()));
        let exec_path = PathBuf::from(Self::form_file_name(self.id.channel, self.id.user, "o"));

        let code = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&code_path)
            .await;

        let output = match code {
            Ok(mut code) => {
                if let Err(_) = code.write_all(self.code.as_bytes()).await {
                    return Err((None, Error::FsError));
                }

                let compile_result = match self.lang {
                    Language::Rust => Self::_compiler_invoke("rustc", &code_path, &exec_path),
                    Language::Cpp => Self::_compiler_invoke("g++", &code_path, &exec_path),
                    Language::C => Self::_compiler_invoke("gcc", &code_path, &exec_path)
                }.await;

                match compile_result {
                    Ok(compile_output) => match Self::_run_exec(&exec_path, timeout).await {
                        Ok(exec_output) => Ok((compile_output, exec_output)),
                        Err(err) => Err((Some(compile_output), err))
                    },
                    Err(err) => Err((None, err))
                }
            },
            Err(_) => Err((None, Error::FsError))
        };

        // TODO: Rewrite
        let _ = fs::remove_file(&code_path).await;
        let _ = fs::remove_file(&exec_path).await;

        info!("Cache files deleted");

        output
    }

    async fn _run_exec(path: &Path, timeout: Duration) -> Result<String, Error> {
        let child = unsafe {
            Command::new(&path.display().to_string())
                .pre_exec(utils::setup_process_env_for_exec)
                .kill_on_drop(true)
                .stderr(Stdio::null())
                .stdin(Stdio::null())
                .stdout(Stdio::piped())
                .spawn()
        };

        match child {
            Ok(child) => tokio::select! {
                output = child.wait_with_output() => {
                    match output {
                        Ok(output) => Ok(String::from_utf8(output.stdout).unwrap()),
                        Err(_) => Err(Error::ExecError)
                    }
                },
                _ = tokio::time::sleep(timeout) => Err(Error::TimeOut)
            },
            Err(_) => Err(Error::BuildError)
        }
    }
    async fn _compiler_invoke(compiler_name: &str, input: &Path, output: &Path) -> Result<String, Error> {
        let child = Command::new(compiler_name)
            .arg(input.display().to_string())
            .arg("-o")
            .arg(output.display().to_string())
            .stdout(Stdio::null())
            .stdin(Stdio::null())
            .stderr(Stdio::piped())
            .spawn();

        match child {
            Ok(child) => match String::from_utf8(child.wait_with_output().await.unwrap().stderr) {
                Ok(out) => Ok(out),
                Err(_) => Err(Error::InvokeError)
            },
            Err(_) => Err(Error::Unsupported)
        }
    }

    fn form_file_name(channel: ChannelId, user: UserId, format: &'static str) -> String {
        format!("./{}-{}.{}", channel.as_u64(), user.as_u64(), format)
    }
}

impl Language {
    const fn as_str(&self) -> &'static str {
        match self {
            Language::Rust => "rs",
            Language::Cpp => "cpp",
            Language::C => "c"
        }
    }
}

impl FromStr for Language {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "rust" => Ok(Self::Rust),
            "cpp" => Ok(Self::Cpp),
            "c" => Ok(Self::C),
            _ => Err(())
        }
    }
}