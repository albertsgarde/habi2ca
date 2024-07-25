mod application;

use anyhow::Result;
use tracing::Level;

pub fn main() -> Result<()> {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    dioxus::launch(application::app);

    Ok(())
}
