use std::path::{Path, PathBuf};

use sea_orm::DatabaseConnection;

pub struct State {
    database: DatabaseConnection,
    database_path: Option<PathBuf>,
}

impl State {
    pub fn new(database: DatabaseConnection, database_path: Option<PathBuf>) -> Self {
        State {
            database,
            database_path,
        }
    }

    pub fn database(&self) -> &DatabaseConnection {
        &self.database
    }

    pub fn database_path(&self) -> Option<&Path> {
        self.database_path.as_deref()
    }
}
