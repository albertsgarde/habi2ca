use std::{path::Path, process::Command};

use anyhow::{Context, Result};
use clap::Args;

#[derive(Args, Debug, Clone)]
pub struct RunFrontend {}

impl RunFrontend {
    pub fn run(&self, workspace_dir: impl AsRef<Path>) -> Result<()> {
        let workspace_dir = workspace_dir.as_ref();
        let frontend_dir = workspace_dir.join("habi2ca-frontend");

        let mut command = Command::new("npm");
        command
            .args(["install"])
            .current_dir(frontend_dir.as_path());
        command
            .spawn()
            .with_context(|| {
                format!(
                    "Failed to install frontend dependencies with frontend path {frontend_dir:?}."
                )
            })?
            .wait()?;

        let mut command = Command::new("npm");
        command
            .args(["run", "dev"])
            .current_dir(frontend_dir.as_path());
        command
            .spawn()
            .with_context(|| {
                format!("Failed to start frontend with frontend path {frontend_dir:?}.")
            })?
            .wait()?;
        Ok(())
    }
}
