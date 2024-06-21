mod database;
mod routes;
mod start;
mod state;
mod table_definitions;

use std::path::{Path, PathBuf};

use actix_web::{
    middleware::{self, TrailingSlash},
    web, HttpServer,
};
use anyhow::{bail, Context, Result};
use database::Database;
use state::State;

#[tokio::main]
pub async fn main() -> Result<()> {
    start::start_server().await?;
    Ok(())
}
