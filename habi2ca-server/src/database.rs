mod player;

use anyhow::{bail, Context, Result};
use std::path::Path;
use tokio_rusqlite::Connection;

use crate::table_definitions;

const TABLES: &[&str] = &[table_definitions::PLAYER_TABLE];

#[derive(Clone)]
pub struct Database {
    connection: Connection,
}

impl Database {
    async fn initialize(connection: Connection) -> Result<Self> {
        assert!(connection
            .call_unwrap(move |connection| {
                let mut statement = connection.prepare("SELECT COUNT(*) FROM sqlite_master")?;
                statement.query_row([], |row| Ok(row.get::<_, i64>(0)? == 0))
            })
            .await
            .with_context(|| "Failed to check if database is empty")?);

        for table in TABLES.iter() {
            connection
                .call(|connection| Ok(connection.execute(table, ())))
                .await?
                .with_context(|| format!("Failed to create table: '{table}'"))?;
        }

        Ok(Database { connection })
    }

    pub async fn create(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        if path.exists() {
            bail!("File already exists: {:?}", path)
        }

        let connection = Connection::open(path)
            .await
            .with_context(|| format!("Failed to create database at '{path:?}'"))?;

        Self::initialize(connection).await
    }

    #[cfg(test)]
    pub async fn create_in_memory() -> Result<Self> {
        let connection = Connection::open_in_memory()
            .await
            .with_context(|| "Failed to create in-memory database")?;

        Self::initialize(connection).await
    }

    pub async fn open(path: impl AsRef<Path>) -> Result<Database> {
        if path.as_ref().exists() {
            let connection = Connection::open(path).await?;
            Ok(Database { connection })
        } else {
            bail!("No such file: {:?}", path.as_ref())
        }
    }
}
