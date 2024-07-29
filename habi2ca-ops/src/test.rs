use std::{path::Path, process::Command};

use anyhow::{bail, Context, Result};
use clap::Args;

#[derive(Args, Debug, Clone)]
pub struct Test {}

impl Test {
    pub fn run(&self, workspace_dir: impl AsRef<Path>) -> Result<()> {
        let workspace_dir = workspace_dir.as_ref();

        let mut command = Command::new(env!("CARGO"));
        command
            .args(["clippy", "--all-targets", "--", "-Dwarnings"])
            .current_dir(workspace_dir);
        let status = command.spawn().context("Failed to run clippy.")?.wait()?;
        if !status.success() {
            bail!("Clippy found warnings.");
        }

        let mut command = Command::new(env!("CARGO"));
        command.args(["fmt", "--check"]).current_dir(workspace_dir);
        let status = command
            .spawn()
            .context("Failed to check formatting.")?
            .wait()?;
        if !status.success() {
            bail!("Formatting incorrect.");
        }

        let mut command = Command::new(env!("CARGO"));
        command.args(["nextest", "run"]).current_dir(workspace_dir);
        command.spawn().context("Failed to run tests.")?.wait()?;
        Ok(())
    }
}
