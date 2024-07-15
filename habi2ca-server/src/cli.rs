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
}

impl ServerConfig {
    pub async fn start(self) -> Result<Never> {
        start::start_server(self).await
    }
}
