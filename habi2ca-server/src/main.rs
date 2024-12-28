mod cli;
mod routes;
mod start;
mod state;

mod logic;
#[cfg(test)]
mod test_utils;
mod tracing;

use std::process::Termination;

use anyhow::Result;
use clap::Parser;
use cli::ServerConfig;

pub enum Never {}

impl Termination for Never {
    fn report(self) -> std::process::ExitCode {
        unreachable!()
    }
}

#[tokio::main]
pub async fn main() -> Result<Never> {
    let server_config = ServerConfig::parse();
    server_config.start().await
}
