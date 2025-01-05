use std::{
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Context, Result};
use clap::Args;

#[derive(Args, Debug, Clone)]
pub struct RunBackend {
    #[arg(default_value = "./local/data/data.db")]
    database_path: String,
    #[arg(default_value = "localhost")]
    hostname: String,
    #[arg(default_value = "8080")]
    port: u16,
    #[arg(long, default_value = "false")]
    release: bool,
    #[arg(long, default_value = "./local/logs/")]
    log_dir: PathBuf,
}

impl RunBackend {
    pub fn run(self, workspace_dir: impl AsRef<Path>) -> Result<()> {
        let workspace_dir = workspace_dir.as_ref();
        let release = self.release;

        let mut command = Command::new(env!("CARGO"));
        command
            .args(["run", "--bin", "habi2ca-server"])
            .current_dir(workspace_dir);
        if release {
            command.arg("--release");
        }
        command.args([
            "--",
            self.database_path.as_str(),
            self.hostname.as_str(),
            self.port.to_string().as_str(),
            "--log-dir",
            self.log_dir.to_str().unwrap(),
        ]);
        command
            .spawn()
            .with_context(|| {
                format!("Failed to start habi2ca-server with workspace path {workspace_dir:?}.")
            })?
            .wait()?;
        Ok(())
    }
}
