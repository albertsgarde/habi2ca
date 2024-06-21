use crate::database::Database;

pub struct State {
    database: Database,
}

impl State {
    pub fn new(database: Database) -> Self {
        State { database }
    }

    pub fn database(&self) -> &Database {
        &self.database
    }
}
