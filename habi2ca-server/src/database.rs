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
    pub async fn initialize(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        if path.exists() {
            bail!("File already exists: {:?}", path)
        }

        let connection = Connection::open(path)
            .await
            .with_context(|| format!("Failed to create database at '{path:?}'"))?;

        for table in TABLES.iter() {
            connection
                .call(|connection| Ok(connection.execute(table, ())))
                .await?
                .with_context(|| format!("Failed to create table: '{table}'"))?;
        }

        Ok(Database { connection })
    }

    pub async fn initialize_in_memory() -> Result<Self> {
        let connection = Connection::open_in_memory()
            .await
            .with_context(|| "Failed to create in-memory database")?;

        for table in TABLES.iter() {
            connection
                .call(|connection| Ok(connection.execute(table, ())))
                .await?
                .with_context(|| format!("Failed to create table: '{table}'"))?;
        }

        Ok(Database { connection })
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
