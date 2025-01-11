use std::fmt::Display;

use anyhow::{Context, Result};
use habi2ca_database::{
    habit::{self, ActiveModel, HabitId, Model},
    player::PlayerId,
};
use sea_orm::{ConnectionTrait, DatabaseConnection, EntityTrait, TransactionTrait};
use serde::{Deserialize, Serialize};

use super::player::Player;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HabitData {
    pub player: PlayerId,
    pub name: String,
    pub description: String,
}

impl HabitData {
    pub fn into_active_model(self) -> ActiveModel {
        ActiveModel {
            player_id: sea_orm::ActiveValue::Set(self.player),
            name: sea_orm::ActiveValue::Set(self.name),
            description: sea_orm::ActiveValue::Set(self.description),
            ..Default::default()
        }
    }
}

#[derive(Debug)]
struct DbError(anyhow::Error);

impl Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for DbError {}

impl From<anyhow::Error> for DbError {
    fn from(error: anyhow::Error) -> Self {
        Self(error)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Habit {
    #[serde(flatten)]
    pub(super) model: Model,
}

impl Habit {
    pub async fn create(db: &impl ConnectionTrait, task_data: HabitData) -> Result<Self> {
        let model = habit::Entity::insert(task_data.into_active_model())
            .exec_with_returning(db)
            .await?;
        Ok(Self { model })
    }

    pub async fn from_id(db: &impl ConnectionTrait, id: HabitId) -> Result<Self> {
        let model = habit::Entity::find_by_id(id)
            .one(db)
            .await
            .with_context(|| format!("Failed to get habit with id {id} from database."))?
            .with_context(|| format!("No habit with id {id} exists."))?;
        Ok(Self { model })
    }

    pub async fn increment(&mut self, db: &DatabaseConnection) -> Result<()> {
        let habit_id = self.model.id;
        let player_id = self.model.player_id;
        let new_model = self.model.clone();
        self.model = db
            .transaction::<_, Model, DbError>(|txn| {
                Box::pin(async move {
                    let mut player = Player::from_id(txn, player_id)
                        .await
                        .with_context(|| format!("Failed to get owner of habit {habit_id}."))?;

                    player.add_xp(txn, 1.0).await.with_context(|| {
                        format!(
                        "Failed to add xp to player {player_id} while completing habit {habit_id}."
                    )
                    })?;
                    Ok(new_model)
                })
            })
            .await
            .context("Failed to complete transaction")?;
        Ok(())
    }

    pub fn id(&self) -> HabitId {
        self.model.id
    }

    pub fn player(&self) -> PlayerId {
        self.model.player_id
    }

    pub fn name(&self) -> &str {
        &self.model.name
    }

    pub fn description(&self) -> &str {
        &self.model.description
    }
}
