mod database;
mod logging;
mod routes;
mod start;
mod state;
mod table_definitions;

use anyhow::Result;

#[tokio::main]
pub async fn main() -> Result<()> {
    logging::init_logging()?;

    start::start_server().await?;
    Ok(())
}
