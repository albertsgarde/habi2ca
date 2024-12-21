use anyhow::{Context, Result};
use habi2ca_database::{
    player::{self, ActiveModel, PlayerId},
    task,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter,
};
use serde::{Deserialize, Serialize};

use super::{level::Level, task::Task};

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    #[serde(flatten)]
    pub(super) model: player::Model,
}

impl Player {
    pub async fn create(db: &impl ConnectionTrait, name: impl AsRef<str>) -> Result<Self> {
        let player = player::new(name);
        Ok(Self {
            model: player::Entity::insert(player)
                .exec_with_returning(db)
                .await
                .context("Failed to insert player into database.")?,
        })
    }

    pub async fn from_id(db: &impl ConnectionTrait, id: PlayerId) -> Result<Self> {
        Ok(Self {
            model: player::Entity::find_by_id(id)
                .one(db)
                .await
                .with_context(|| {
                    format!("Failure while getting player with id {id} from database.")
                })?
                .with_context(|| format!("Player with id {id} not found in database."))?,
        })
    }

    #[cfg(test)]
    pub fn id(&self) -> PlayerId {
        self.model.id
    }

    pub async fn add_xp(&mut self, db: &impl ConnectionTrait, xp_delta: f64) -> Result<()> {
        self.model.xp += xp_delta;
        loop {
            let xp_needed = Level::from_id(db, self.model.level).await?.xp_requirement();
            if self.model.xp >= xp_needed {
                self.model.xp -= xp_needed;
                self.model.level = self.model.level.next_level();
            } else {
                break;
            }
        }

        let active_model = ActiveModel {
            xp: ActiveValue::Set(self.model.xp),
            level: ActiveValue::Set(self.model.level),
            ..self.model.clone().into_active_model()
        };
        active_model.update(db).await.with_context(|| {
            format!(
                "Failure while updating player '{}' in database.",
                self.model.id
            )
        })?;
        Ok(())
    }

    pub async fn get_tasks(&self, db: &DatabaseConnection) -> Result<Vec<Task>> {
        let id = self.model.id;
        let models = task::Entity::find()
            .filter(task::Column::PlayerId.eq(id))
            .all(db)
            .await
            .with_context(|| format!("Failed to get tasks for player '{id}'"))?;

        Ok(models.into_iter().map(|model| Task { model }).collect())
    }
}
