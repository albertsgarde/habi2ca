use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

use crate::{
    start::{self},
    Never,
};

#[derive(Parser, Debug, Clone)]
pub struct ServerConfig {
    pub database_path: PathBuf,
    pub hostname: String,
    pub port: u16,
    // Clear database and reapply all migrations if pending migrations cannot be applied.
    #[clap(long)]
    pub force_migrations: bool,
    #[clap(long)]
    pub log_dir: Option<PathBuf>,
}

impl ServerConfig {
    pub async fn start(self) -> Result<Never> {
        start::start_server(self).await
    }
}
