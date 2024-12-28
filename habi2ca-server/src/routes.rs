mod levels;
mod players;
mod tasks;

use actix_web::{web, ResponseError, Scope};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RouteError {
    #[error("{0:?}\n")]
    Anyhow(anyhow::Error),
}

impl From<anyhow::Error> for RouteError {
    fn from(error: anyhow::Error) -> Self {
        RouteError::Anyhow(error)
    }
}

impl ResponseError for RouteError {}

pub fn add_routes(scope: Scope) -> Scope {
    scope
        .service(players::add_routes(web::scope("/players")))
        .service(tasks::add_routes(web::scope("/tasks")))
        .service(levels::add_routes(web::scope("/levels")))
}
