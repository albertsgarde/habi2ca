mod admin;
mod habits;
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
        .service(admin::add_routes(web::scope("/admin")))
        .service(players::add_routes(web::scope("/players")))
        .service(tasks::add_routes(web::scope("/tasks")))
        .service(habits::add_routes(web::scope("/habits")))
        .service(levels::add_routes(web::scope("/levels")))
}
