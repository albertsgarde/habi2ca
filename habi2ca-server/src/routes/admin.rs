use actix_web::{get, web, Responder, Scope};
use tracing::info;

use crate::{database_utils, routes::RouteError, state::State};

#[get("reinitialize-database")]
pub async fn reinitialize_database(state: web::Data<State>) -> Result<impl Responder, RouteError> {
    info!("Reinitializing database...");
    if let Some(database_path) = state.database_path() {
        database_utils::reinitialize_database(database_path).await?;
    }
    info!("Database reinitialized.");
    Ok(("", actix_web::http::StatusCode::OK))
}

pub fn add_routes(scope: Scope) -> Scope {
    scope.service(reinitialize_database)
}
