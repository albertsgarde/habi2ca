use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

fn player_table() -> TableCreateStatement {
    Table::create()
        .table(Player::Table)
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
        .col(
            ColumnDef::new(Player::Level)
                .integer()
                .not_null()
                .check(Expr::col(Player::Level).gte(1)),
        )
        .to_owned()
}

fn task_table() -> TableCreateStatement {
    Table::create()
        .table(Task::Table)
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
        .to_owned()
}

fn level_table() -> TableCreateStatement {
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
        .to_owned()
}

async fn level_seed_data(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
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

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(player_table()).await?;
        manager.create_table(task_table()).await?;
        manager.create_table(level_table()).await?;

        level_seed_data(manager).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Level::Table).to_owned())
            .await?;
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
    Level,
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

#[derive(DeriveIden)]
enum Level {
    Table,
    Id,
    XpRequirement,
}
