use std::{fs, path::Path};

use actix_web::{
    dev::{ServiceFactory, ServiceRequest},
    middleware::{self, TrailingSlash},
    web, App, HttpServer,
};
use anyhow::{bail, Context, Result};
use habi2ca_database::migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};

use crate::{cli::ServerConfig, routes, state::State, Never};

pub async fn open_or_initialize_database(
    database_path: impl AsRef<Path>,
) -> Result<DatabaseConnection> {
    let database_path = database_path.as_ref();
    let database_url = if database_path.exists() {
        format!("sqlite:{}?mode=rw", database_path.display())
    } else {
        format!("sqlite:{}?mode=rwc", database_path.display())
    };
    let database = Database::connect(database_url.as_str())
        .await
        .with_context(|| {
            format!(
                "Failed to connect to database at '{}'.",
                database_url.as_str()
            )
        })?;
    if !Migrator::get_pending_migrations(&database)
        .await
        .context("Failed to get pending migrations")?
        .is_empty()
    {
        println!("Pending migrations found. Creating backup and running migrations...");
        let backup_path = database_path.with_extension("bak");
        if backup_path.exists() {
            bail!("Backup file already exists at '{}'.", backup_path.display());
        }
        fs::copy(database_path, &backup_path)?;
        Migrator::up(&database, None)
            .await
            .context("Failed to run pending migrations")?;
        println!("Migrations complete.");
    }
    Ok(database)
}

pub fn create_app(
    database: DatabaseConnection,
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

    let server = HttpServer::new(move || create_app(database.clone()));

    println!("Starting server at http://{hostname}:{port}");
    server.bind((hostname, port))?.run().await?;

    bail!("Server stopped unexpectedly.")
}
