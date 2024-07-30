pub use sea_orm_migration::MigratorTrait;
use sea_orm_migration::{async_trait::async_trait, MigrationTrait};

mod m20240727_133538_initial;
mod m20240730_133138_level;
pub struct Migrator;

#[async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240727_133538_initial::Migration),
            Box::new(m20240730_133138_level::Migration),
        ]
    }
}
