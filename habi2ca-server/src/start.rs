use std::{fs, path::PathBuf};

use ::tracing::info;
use actix_web::{
    body::MessageBody,
    dev::{ServiceFactory, ServiceRequest},
    middleware::{self, TrailingSlash},
    web, App, HttpServer,
};
use anyhow::{bail, Result};
use sea_orm::DatabaseConnection;

use crate::{cli::ServerConfig, database_utils, routes, state::State, tracing, Never};

pub fn create_app_with_database_path(
    database: DatabaseConnection,
    database_path: Option<PathBuf>,
) -> App<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse<impl MessageBody>,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    App::new()
        .app_data(web::Data::new(State::new(database, database_path)))
        .wrap(middleware::NormalizePath::new(TrailingSlash::Trim))
        .service(routes::add_routes(web::scope("/api")))
}

#[cfg(test)]
pub fn create_app(
    database: DatabaseConnection,
) -> App<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse<impl MessageBody>,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    create_app_with_database_path(database, None)
}

pub async fn start_server(config: ServerConfig) -> Result<Never> {
    let ServerConfig {
        database_path,
        hostname,
        port,
        force_migrations,
        log_dir,
    } = config;

    let _guard = log_dir.map(tracing::setup_tracing).transpose()?;

    let hostname = hostname.as_ref();
    fs::create_dir_all(database_path.parent().unwrap())?;
    let database =
        database_utils::open_or_initialize_database(&database_path, force_migrations).await?;

    let server = HttpServer::new(move || {
        create_app_with_database_path(database.clone(), Some(database_path.clone()))
    });

    info!("Starting server at http://{hostname}:{port}");
    server.bind((hostname, port))?.run().await?;

    bail!("Server stopped unexpectedly.")
}
