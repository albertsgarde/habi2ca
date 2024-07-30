use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Level::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Level::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key()
                            .check(Expr::col(Level::Id).gte(1)),
                    )
                    .col(
                        ColumnDef::new(Level::XpRequirement)
                            .float()
                            .not_null()
                            .check(Expr::col(Level::XpRequirement).gt(0.0)),
                    )
                    .to_owned(),
            )
            .await?;

        let xp_requirements: Vec<f64> =
            serde_json::from_str(include_str!("../../../gamedata/levels.json"))
                .expect("Failed to parse levels.json");

        for (index, xp) in xp_requirements.into_iter().enumerate() {
            let level = index as i64 + 1;
            let insert = Query::insert()
                .into_table(Level::Table)
                .columns([Level::Id, Level::XpRequirement])
                .values([
                    SimpleExpr::Value(Value::BigInt(Some(level))),
                    SimpleExpr::Value(Value::Double(Some(xp))),
                ])
                .expect("Error in insert query.")
                .to_owned();
            manager.exec_stmt(insert).await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Level::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Level {
    Table,
    Id,
    XpRequirement,
}
