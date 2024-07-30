use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Player::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Player::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Player::Name).string().not_null())
                    .col(
                        ColumnDef::new(Player::Xp)
                            .float()
                            .not_null()
                            .check(Expr::col(Player::Xp).gte(0.0)),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Task::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Task::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Task::PlayerId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_player_id")
                            .from(Task::Table, Task::PlayerId)
                            .to(Player::Table, Player::Id),
                    )
                    .col(ColumnDef::new(Task::Name).string().not_null())
                    .col(ColumnDef::new(Task::Description).string().not_null())
                    .col(ColumnDef::new(Task::Completed).boolean().not_null())
                    .to_owned(),
            )
            .await?;

        let insert = Query::insert()
            .into_table(Player::Table)
            .columns([Player::Name, Player::Xp])
            .values([
                SimpleExpr::Value(Value::String(Some(Box::new("Alice".to_owned())))),
                SimpleExpr::Value(Value::Double(Some(0.))),
            ])
            .expect("Error in insert query.")
            .to_owned();
        manager.exec_stmt(insert).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Task::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Player::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Player {
    Table,
    Id,
    Name,
    Xp,
}

#[derive(DeriveIden)]
enum Task {
    Table,
    Id,
    PlayerId,
    Name,
    Description,
    Completed,
}
