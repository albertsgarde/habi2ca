use sea_orm_migration::{async_trait::async_trait, MigrationTrait};

pub use sea_orm_migration::MigratorTrait;

mod m20240727_133538_initial;
pub struct Migrator;

#[async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20240727_133538_initial::Migration)]
    }
}
