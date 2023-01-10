use std::{str::FromStr, path::{PathBuf, Path}, process::Stdio};
use tokio::{
    fs,
    process::Command
};
use serenity::model::id::{ChannelId, UserId};
use tokio::io::AsyncWriteExt;

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
    Unsupported
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

    pub async fn compile_and_run(self) -> Result<(String, String), (Option<String>, Error)> {
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
                    Ok(compile_output) => match Self::_run_exec(&exec_path).await {
                        Ok(exec_output) => Ok((compile_output, exec_output)),
                        Err(err) => Err((Some(compile_output), err))
                    },
                    Err(err) => Err((None, err))
                }
            },
            Err(_) => Err((None, Error::FsError))
        };

        // TODO: Rewrite
        fs::remove_file(&code_path).await.unwrap();
        fs::remove_file(&exec_path).await.unwrap();

        info!("Cache files deleted");

        output
    }

    async fn _run_exec(path: &Path) -> Result<String, Error> {
        let child = unsafe {
            Command::new(&path.display().to_string())
                .pre_exec(setup_process_env_for_exec)
                .kill_on_drop(true)
                .stderr(Stdio::null())
                .stdin(Stdio::null())
                .stdout(Stdio::piped())
                .spawn()
        };

        match child {
            Ok(child) => match child.wait_with_output().await {
                Ok(out) => Ok(String::from_utf8(out.stdout).unwrap()),
                Err(_) => Err(Error::ExecError)
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

use libc::{rlimit, nice};
use tracing::info;

fn setup_process_env_for_exec() -> std::io::Result<()> {
    let rlim_as = rlimit {
        rlim_cur: 10485760,
        rlim_max: 10485760
    };
    let rlim_core = rlimit {
        rlim_cur: 0,
        rlim_max: 0
    };
    let data_seg = rlimit {
        rlim_cur: 5242880,
        rlim_max: 5242880
    };
    let max_file_size = rlimit {
        rlim_cur: 0,
        rlim_max: 0
    };
    let nproc = rlimit {
        rlim_cur: 2,
        rlim_max: 2
    };
    let stack = rlimit {
        rlim_cur: 2097152,
        rlim_max: 2097152
    };

    unsafe {
        libc::setrlimit(libc::RLIMIT_AS, &rlim_as);
        libc::setrlimit(libc::RLIMIT_CORE, &rlim_core);
        libc::setrlimit(libc::RLIMIT_DATA, &data_seg);
        libc::setrlimit(libc::RLIMIT_FSIZE, &max_file_size);
        libc::setrlimit(libc::RLIMIT_NPROC, &nproc);
        libc::setrlimit(libc::RLIMIT_STACK, &stack);

        libc::perror("setrlimit\0".as_ptr() as *const libc::c_char);

        nice(15.into());

        libc::perror("nice\0".as_ptr() as *const libc::c_char);
    }

    Ok(())
}