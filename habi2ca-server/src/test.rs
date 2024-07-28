use habi2ca_database::migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};

pub async fn setup_database() -> DatabaseConnection {
    let database = Database::connect("sqlite::memory:")
        .await
        .expect("Failed to connect to database.");

    Migrator::up(&database, None)
        .await
        .expect("Failed to run migrations.");

    database
}

#[cfg(test)]
mod test {

    #[tokio::test]
    async fn setup_test_database() {
        let _ = super::setup_database().await;
    }
}
