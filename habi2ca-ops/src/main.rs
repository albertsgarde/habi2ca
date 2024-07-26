use std::{path::PathBuf, process::Command};

use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand};
use habi2ca_ops as ops;

#[derive(Parser, Debug, Clone)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Backend(RunBackend),
    Frontend(RunFrontend),
}

#[derive(Args, Debug, Clone)]
struct RunBackend {
    #[arg(long, default_value = "false")]
    release: bool,
    #[arg(last = true)]
    backend_args: Vec<String>,
}

#[derive(Args, Debug, Clone)]
struct RunFrontend {}

fn run_backend(cli: RunBackend, workspace_dir: &PathBuf) -> Result<()> {
    let release = cli.release;

    let mut command = Command::new(env!("CARGO"));
    command
        .args(["run", "--bin", "habi2ca-server"])
        .current_dir(workspace_dir.as_path());
    if release {
        command.arg("--release");
    }
    command.args(cli.backend_args);
    command
        .spawn()
        .with_context(|| {
            format!("Failed to start habi2ca-server with workspace path {workspace_dir:?}.")
        })?
        .wait()?;
    Ok(())
}

fn run_frontend(_cli: RunFrontend, workspace_dir: &PathBuf) -> Result<()> {
    let frontend_dir = workspace_dir.join("habi2ca-frontend");

    let mut command = Command::new("npm");
    command
        .args(["install"])
        .current_dir(frontend_dir.as_path());
    command
        .spawn()
        .with_context(|| {
            format!("Failed to install frontend dependencies with frontend path {frontend_dir:?}.")
        })?
        .wait()?;

    let mut command = Command::new("npm");
    command
        .args(["run", "dev"])
        .current_dir(frontend_dir.as_path());
    command
        .spawn()
        .with_context(|| format!("Failed to start frontend with frontend path {frontend_dir:?}."))?
        .wait()?;
    Ok(())
}

pub fn main() -> Result<()> {
    let cli = Cli::parse();
    let workspace_dir = ops::workspace_dir()?;
    match cli.command {
        Commands::Backend(backend) => run_backend(backend, &workspace_dir),
        Commands::Frontend(frontend) => run_frontend(frontend, &workspace_dir),
    }
}
