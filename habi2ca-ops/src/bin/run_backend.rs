use std::process::Command;

use anyhow::{Context, Result};
use clap::Parser;
use habi2ca_ops as ops;

#[derive(Parser, Debug, Clone)]
struct Cli {
    #[arg(long, default_value = "false")]
    release: bool,
}

pub fn main() -> Result<()> {
    let cli = Cli::parse();
    let release = cli.release;

    let workspace_dir = ops::workspace_dir()?;
    ops::trunk_build(workspace_dir.join("habi2ca-frontend"), release)?;
    let mut command = Command::new(env!("CARGO"));
    command
        .args(["run", "--bin", "habi2ca-server"])
        .current_dir(workspace_dir.as_path());
    if release {
        command.arg("--release");
    }
    command
        .spawn()
        .with_context(|| {
            format!("Failed to start habi2ca-server with workspace path {workspace_dir:?}.")
        })?
        .wait()?;
    Ok(())
}
