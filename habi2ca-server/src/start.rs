use std::{
    fs,
    path::{Path, PathBuf},
};

use actix_service::{IntoServiceFactory, Service};
use actix_web::{
    dev::{ServiceFactory, ServiceRequest},
    middleware::{self, TrailingSlash},
    web, App, HttpServer,
};
use anyhow::{bail, Context, Result};

use crate::{database::Database, routes, state::State};

pub enum Empty {}

pub async fn open_or_initialize_database(database_path: impl AsRef<Path>) -> Result<Database> {
    let database_path = database_path.as_ref();
    if database_path.exists() {
        Database::open(database_path)
            .await
            .with_context(|| format!("Failed to open database at '{database_path:?}'"))
    } else {
        Database::initialize(database_path)
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
        .service(
            web::scope("/api")
                .service(routes::create_player)
                .service(routes::add_xp)
                .service(routes::get_player),
        )
}

pub async fn start_server() -> Result<Empty> {
    let database_path = PathBuf::from("local/data.db");
    if database_path.exists() {
        fs::remove_file(database_path.as_path())?;
    }
    let database = open_or_initialize_database(&database_path).await?;
    database.create_player("Alice").await?;

    let url = "localhost";
    let port = 8080;

    let server = HttpServer::new(move || create_app(database.clone()));

    println!("Starting server at http://{}:{}", url, port);
    server.bind((url, port))?.run().await?;

    bail!("Server stopped unexpectedly.")
}
