use anyhow::{Context, Result};
use habi2ca_database::migration::{Migrator, MigratorTrait};
use sea_orm::{sqlx::types::chrono, Database, DatabaseConnection};
use std::{
    fs,
    path::{Path, PathBuf},
};
use tracing::info;

pub fn sqlite_url(database_path: impl AsRef<Path>) -> String {
    format!("sqlite:{}?mode=rw", database_path.as_ref().display())
}

pub async fn sqlite_connection(database_path: impl AsRef<Path>) -> Result<DatabaseConnection> {
    Database::connect(sqlite_url(database_path.as_ref()).as_str())
        .await
        .with_context(|| {
            format!(
                "Failed to connect to database at '{}'.",
                database_path.as_ref().display()
            )
        })
}

pub fn backup(database_path: &Path) -> Result<PathBuf> {
    let timestamp = chrono::Utc::now().format("%Y%m%d%H%M%S").to_string();
    let file_name = database_path.file_stem().unwrap().to_string_lossy();
    let new_path = database_path.with_file_name(format!("{}{}", timestamp, file_name));
    let new_path = if let Some(extension) = database_path.extension() {
        new_path.with_extension(extension)
    } else {
        new_path
    };
    fs::copy(database_path, &new_path).with_context(|| {
        format!("Failed to move database at '{database_path:?}' to new path '{new_path:?}'")
    })?;
    info!("Database backup created at '{new_path:?}'");
    Ok(new_path)
}

pub async fn reinitialize_database(database_path: &Path) -> Result<PathBuf> {
    let backup_path = backup(database_path)?;
    let database = sqlite_connection(database_path).await?;
    Migrator::fresh(&database)
        .await
        .context("Failed to run fresh migrations")?;
    Ok(backup_path)
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
        if force_migrations {
            if Migrator::up(&database, None).await.is_err() {
                reinitialize_database(database_path).await.map(|_| ())
            } else {
                Ok(())
            }
        } else {
            Migrator::up(&database, None)
                .await
                .context("Failed to run pending migrations")
        }?;
        info!("Migrations complete.");
    }
    Ok(database)
}
