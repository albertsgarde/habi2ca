use std::{path::Path, process::Command};

use anyhow::{Context, Result};
use clap::Args;

#[derive(Args, Debug, Clone)]
pub struct RunBackend {
    #[arg(long, default_value = "false")]
    release: bool,
    #[arg(last = true)]
    backend_args: Vec<String>,
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
        command.args(self.backend_args);
        command
            .spawn()
            .with_context(|| {
                format!("Failed to start habi2ca-server with workspace path {workspace_dir:?}.")
            })?
            .wait()?;
        Ok(())
    }
}
