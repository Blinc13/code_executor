use std::env;
use std::path::PathBuf;
use tokio::process::Command;

impl super::Compiler {
    pub(crate) async fn _compile_rust(self) -> Result<(PathBuf, Vec<u8>), ()> {
        let exec_path = format!(
            "./{}",
            Self::form_file_name(self.id.0, self.id.1, "o")
        );

        let output = Command::new("rustc")
            .arg(self.path)
            .arg("-o")
            .arg(&exec_path)
            .output().await;

        match output {
            Ok(out) => Ok((exec_path.into(), out.stderr)),
            Err(_) => Err(())
        }
    }
}