use std::{
    io::Write,
    str::FromStr,
    path::PathBuf,
    fs::{File, OpenOptions}
};
use serenity::model::id::{ChannelId, UserId};

mod c;
mod cpp;
mod rust;

#[derive(Copy, Clone)]
pub enum Language {
    Rust,
    Cpp,
    C
}

pub struct Compiler {
    file: File,
    path: PathBuf,
    lang: Language,
    id: (ChannelId, UserId)
}

impl Compiler {
    pub fn new(lang: Language, code: String, channel: ChannelId, user: UserId) -> Self {
        let mut path: PathBuf = format!("./{}", Self::form_file_name(channel, user, lang.as_str())).into();

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path).unwrap();

        file.write_all(code.as_bytes()).unwrap();

        Self {
            file,
            path,
            lang,
            id: (channel, user)
        }
    }

    pub async fn compile(mut self) -> Result<(PathBuf, Vec<u8>), ()> {
        match self.lang {
            Language::Rust => self._compile_rust().await,
            Language::Cpp => self._compile_cpp().await,
            Language::C => self._compile_c().await
        }
    }

    fn form_file_name(channel: ChannelId, user: UserId, format: &'static str) -> String {
        format!("{}-{}.{}", channel.as_u64(), user.as_u64(), format)
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