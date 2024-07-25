use std::{fs, path::Path};

use actix_web::{
    dev::{ServiceFactory, ServiceRequest},
    middleware::{self, TrailingSlash},
    web, App, HttpServer,
};
use anyhow::{bail, Context, Result};

use crate::{cli::ServerConfig, database::Database, routes, state::State, Never};

pub async fn open_or_initialize_database(database_path: impl AsRef<Path>) -> Result<Database> {
    let database_path = database_path.as_ref();
    if database_path.exists() {
        Database::open(database_path)
            .await
            .with_context(|| format!("Failed to open database at '{database_path:?}'"))
    } else {
        Database::create(database_path)
            .await
            .with_context(|| format!("Failed to initialize database at '{database_path:?}'"))
    }
}

pub fn create_app(
    database: Database,
) -> App<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    App::new()
        .app_data(web::Data::new(State::new(database)))
        .wrap(middleware::NormalizePath::new(TrailingSlash::Trim))
        .service(routes::add_routes(web::scope("/api")))
}

pub async fn start_server(config: ServerConfig) -> Result<Never> {
    let ServerConfig {
        database_path,
        hostname,
        port,
    } = config;
    let hostname = hostname.as_ref();
    fs::create_dir_all(database_path.parent().unwrap())?;
    let database = open_or_initialize_database(&database_path).await?;
    database.create_player("Alice").await?;

    let server = HttpServer::new(move || create_app(database.clone()));

    println!("Starting server at http://{hostname}:{port}");
    server.bind((hostname, port))?.run().await?;

    bail!("Server stopped unexpectedly.")
}
