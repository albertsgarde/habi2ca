mod backend;
mod docker;
mod frontend;
mod utils;

use anyhow::Result;
use clap::{Parser, Subcommand};

use backend::RunBackend;
use docker::Docker;
use frontend::RunFrontend;

#[derive(Parser, Debug, Clone)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Backend(RunBackend),
    Frontend(RunFrontend),
    Docker(Docker),
}

pub fn main() -> Result<()> {
    let cli = Cli::parse();
    let workspace_dir = utils::workspace_dir()?;
    match cli.command {
        Commands::Backend(backend) => backend.run(workspace_dir),
        Commands::Frontend(frontend) => frontend.run(workspace_dir),
        Commands::Docker(docker) => docker.run(workspace_dir),
    }
}
