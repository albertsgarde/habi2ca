mod database;
mod routes;
mod start;
mod state;
mod table_definitions;

use anyhow::Result;

#[tokio::main]
pub async fn main() -> Result<()> {
    start::start_server().await?;
    Ok(())
}
