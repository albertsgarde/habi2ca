use sea_orm::DatabaseConnection;

pub struct State {
    database: DatabaseConnection,
}

impl State {
    pub fn new(database: DatabaseConnection) -> Self {
        State { database }
    }

    pub fn database(&self) -> &DatabaseConnection {
        &self.database
    }
}
