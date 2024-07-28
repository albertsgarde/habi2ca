use std::{
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Context, Result};
use clap::{command, Args, Subcommand};

#[derive(Args, Debug, Clone)]
pub struct Docker {
    #[command(subcommand)]
    command: Commands,
}

impl Docker {
    pub fn run(self, workspace_dir: impl AsRef<Path>) -> Result<()> {
        match self.command {
            Commands::Build(build) => build.run(workspace_dir),
            Commands::Up(up) => up.run(workspace_dir),
            Commands::Publish(publish) => publish.run(),
        }
    }
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Build(Build),
    Up(Up),
    Publish(Publish),
}

fn base_compose_command(workspace_dir: impl AsRef<Path>) -> Command {
    let mut command = Command::new("docker");
    command
        .args(["compose", "-f", "docker/compose.yaml"])
        .current_dir(workspace_dir.as_ref());
    command
}

#[derive(Args, Debug, Clone)]
struct Build {}

impl Build {
    fn run(self, workspace_dir: impl AsRef<Path>) -> Result<()> {
        let mut command = base_compose_command(workspace_dir);
        // DB_DIR doesn't do anything when we build, but has to be there for the compose file to work.
        command.arg("build").env("DB_DIR", "./local/db/");

        command
            .spawn()
            .with_context(|| format!("Failed to build docker containers."))?
            .wait()?;
        Ok(())
    }
}

#[derive(Args, Debug, Clone)]
struct Up {
    #[arg(long, short, default_value = "./local/db/")]
    database_path: String,
}

impl Up {
    fn run(self, workspace_dir: impl AsRef<Path>) -> Result<()> {
        let database_path = PathBuf::from(self.database_path.as_str());
        if !database_path.exists() {
            std::fs::create_dir_all(&database_path)?;
        }

        let mut command = base_compose_command(workspace_dir);
        command.arg("up").env("DB_DIR", self.database_path.as_str()); // Doesn't do anything when we build, but has to be there for the compose file to work.

        command
            .spawn()
            .with_context(|| format!("Failed to start docker containers."))?
            .wait()?;
        Ok(())
    }
}

#[derive(Args, Debug, Clone)]
struct Publish {
    #[arg()]
    version: String,
}

impl Publish {
    fn run(self) -> Result<()> {
        let backend_image = format!("albertgarde/habi2ca-backend:{}", self.version);
        let frontend_image = format!("albertgarde/habi2ca-frontend:{}", self.version);

        let mut command = Command::new("docker");
        command.args(["tag", "habi2ca-backend", backend_image.as_str()]);
        command
            .spawn()
            .with_context(|| format!("Failed to tag backend docker container."))?
            .wait()?;

        let mut command = Command::new("docker");
        command.args(["tag", "habi2ca-frontend", frontend_image.as_str()]);
        command
            .spawn()
            .with_context(|| format!("Failed to tag frontend docker container."))?
            .wait()?;

        let mut command = Command::new("docker");
        command.args(["push", backend_image.as_str()]);
        command
            .spawn()
            .with_context(|| format!("Failed to push backend docker container."))?
            .wait()?;

        let mut command = Command::new("docker");
        command.args(["push", frontend_image.as_str()]);
        command
            .spawn()
            .with_context(|| format!("Failed to push frontend docker container."))?
            .wait()?;
        Ok(())
    }
}
