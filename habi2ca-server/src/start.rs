use std::{
    fs,
    path::{Path, PathBuf},
};

use ::tracing::info;
use actix_web::{
    dev::{ServiceFactory, ServiceRequest},
    middleware::{self, TrailingSlash},
    web, App, HttpServer,
};
use anyhow::{bail, Context, Result};
use habi2ca_database::migration::{Migrator, MigratorTrait};
use sea_orm::{sqlx::types::chrono, Database, DatabaseConnection};

use crate::{cli::ServerConfig, routes, state::State, tracing, Never};

fn backup(database_path: &Path) -> Result<PathBuf> {
    let new_path =
        database_path.with_file_name(chrono::Utc::now().format("%Y%m%d%H%M%S").to_string());
    fs::copy(database_path, &new_path).with_context(|| {
        format!("Failed to move database at '{database_path:?}' to new path '{new_path:?}'")
    })?;
    Ok(new_path)
}

pub async fn open_or_initialize_database(
    database_path: impl AsRef<Path>,
    force_migrations: bool,
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
        info!("Pending migrations found. Creating backup and running migrations...");
        let backup_path = backup(database_path)?;
        if force_migrations {
            if Migrator::up(&database, None).await.is_err() {
                Migrator::fresh(&database)
                    .await
                    .context("Failed to run fresh migrations")
            } else {
                Ok(())
            }
        } else {
            Migrator::up(&database, None)
                .await
                .context("Failed to run pending migrations")
        }
        .map_err(|e| {
            let _ = fs::remove_file(backup_path);
            e
        })?;
        info!("Migrations complete.");
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
        force_migrations,
        log_dir,
    } = config;

    let _guard = log_dir.map(tracing::setup_tracing).transpose()?;

    let hostname = hostname.as_ref();
    fs::create_dir_all(database_path.parent().unwrap())?;
    let database = open_or_initialize_database(&database_path, force_migrations).await?;

    let server = HttpServer::new(move || create_app(database.clone()));

    info!("Starting server at http://{hostname}:{port}");
    server.bind((hostname, port))?.run().await?;

    bail!("Server stopped unexpectedly.")
}
