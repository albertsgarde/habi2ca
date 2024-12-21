use anyhow::{Context, Result};

use habi2ca_database::level::{self, LevelId, Model};
use sea_orm::{ConnectionTrait, DatabaseConnection, EntityTrait};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Level {
    #[serde(flatten)]
    pub(super) model: Model,
}

impl Level {
    pub async fn from_id(database: &impl ConnectionTrait, id: LevelId) -> Result<Self> {
        Ok(Self {
            model: level::Entity::find_by_id(id)
                .one(database)
                .await
                .with_context(|| format!("Failed to get level with id {id} from database."))?
                .with_context(|| format!("No level with id {id} exists."))?,
        })
    }

    pub async fn all_levels(database: &DatabaseConnection) -> Result<Vec<Level>> {
        let models = level::Entity::find()
            .all(database)
            .await
            .context("Failed to get all levels from database.")?;
        Ok(models.into_iter().map(|model| Level { model }).collect())
    }

    pub fn xp_requirement(&self) -> f64 {
        self.model.xp_requirement
    }
}
