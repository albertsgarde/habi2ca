use anyhow::{Context, Result};
use habi2ca_database::{
    level,
    player::{self, ActiveModel, PlayerId},
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ConnectionTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, Set,
};
use serde::{Deserialize, Serialize};

use super::level::Level;

#[cfg(test)]
use habi2ca_database::level::LevelId;

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    #[serde(flatten)]
    pub(super) model: player::Model,
    pub(super) xp_requirement: f64,
}

impl Player {
    fn default_model(name: impl AsRef<str>) -> ActiveModel {
        ActiveModel {
            name: Set(name.as_ref().to_owned()),
            xp: Set(0.0),
            level_id: Set(1.into()),
            ..Default::default()
        }
    }

    pub async fn create(db: &impl ConnectionTrait, name: impl AsRef<str>) -> Result<Self> {
        let player = Self::default_model(name);
        let model = player::Entity::insert(player)
            .exec_with_returning(db)
            .await
            .context("Failed to insert player into database.")?;
        let level = Level::from_id(db, model.level_id)
            .await
            .context("Failed to get level from database.")?;
        Ok(Self {
            model,
            xp_requirement: level.xp_requirement(),
        })
    }

    pub async fn from_id(db: &impl ConnectionTrait, id: PlayerId) -> Result<Self> {
        let (player_model, level_model_option) = player::Entity::find_by_id(id)
            .find_also_related(level::Entity)
            .one(db)
            .await
            .with_context(|| format!("Failure while getting player with id {id} from database."))?
            .with_context(|| format!("Player with id {id} not found in database."))?;
        let level_model = level_model_option.with_context(|| {
            format!(
                "Player {id}s level ({}) not found in database.",
                player_model.level_id
            )
        })?;
        Ok(Self {
            model: player_model,
            xp_requirement: level_model.xp_requirement,
        })
    }

    pub async fn all(db: &DatabaseConnection) -> Result<Vec<Self>> {
        let models = player::Entity::find()
            .find_also_related(level::Entity)
            .all(db)
            .await
            .context("Failed to get players from database.")?;
        models
            .into_iter()
            .map(|(player_model, level_model)| {
                let level_model = level_model.with_context(|| {
                    format!(
                        "Player {}s level ({}) not found in database.",
                        player_model.id, player_model.level_id
                    )
                })?;
                Ok(Self {
                    model: player_model,
                    xp_requirement: level_model.xp_requirement,
                })
            })
            .collect::<Result<Vec<Self>>>()
    }

    #[cfg(test)]
    pub fn id(&self) -> PlayerId {
        self.model.id
    }

    #[cfg(test)]
    pub fn name(&self) -> &str {
        &self.model.name
    }

    #[cfg(test)]
    pub fn xp(&self) -> f64 {
        self.model.xp
    }

    #[cfg(test)]
    pub fn level(&self) -> LevelId {
        self.model.level_id
    }

    pub fn xp_requirement(&self) -> f64 {
        self.xp_requirement
    }

    pub async fn add_xp(&mut self, db: &impl ConnectionTrait, xp_delta: f64) -> Result<()> {
        self.model.xp += xp_delta;
        loop {
            let xp_needed = self.xp_requirement();
            if self.model.xp >= xp_needed {
                self.model.xp -= xp_needed;
                self.model.level_id = self.model.level_id.next_level();
            } else {
                break;
            }
        }

        let active_model = ActiveModel {
            xp: ActiveValue::Set(self.model.xp),
            level_id: ActiveValue::Set(self.model.level_id),
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
}
