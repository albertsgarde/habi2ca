use std::fmt::Display;

use crate::logic::player::Player;
use anyhow::{Context, Result};
use habi2ca_database::{
    player::PlayerId,
    task::{self, ActiveModel, Model, TaskId},
};
use sea_orm::{
    ConnectionTrait, DatabaseConnection, EntityTrait, IntoActiveModel, TransactionTrait,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TaskData {
    pub player: PlayerId,
    pub name: String,
    pub description: String,
    pub completed: bool,
}

impl TaskData {
    pub fn into_active_model(self) -> task::ActiveModel {
        task::ActiveModel {
            player_id: sea_orm::ActiveValue::Set(self.player),
            name: sea_orm::ActiveValue::Set(self.name),
            description: sea_orm::ActiveValue::Set(self.description),
            completed: sea_orm::ActiveValue::Set(self.completed),
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
pub struct Task {
    #[serde(flatten)]
    pub(super) model: Model,
}

impl Task {
    pub async fn create(db: &impl ConnectionTrait, task_data: TaskData) -> Result<Self> {
        let model = task::Entity::insert(task_data.into_active_model())
            .exec_with_returning(db)
            .await
            .context("Failed to insert task into database.")?;

        Ok(Task { model })
    }

    pub async fn from_id(db: &impl ConnectionTrait, id: TaskId) -> Result<Self> {
        Ok(Self {
            model: task::Entity::find_by_id(id)
                .one(db)
                .await
                .with_context(|| format!("Failed to get task with id {id} from database."))?
                .with_context(|| format!("No task with id {id} exists."))?,
        })
    }

    #[cfg(test)]
    pub fn id(&self) -> TaskId {
        self.model.id
    }

    #[cfg(test)]
    pub fn player_id(&self) -> PlayerId {
        self.model.player_id
    }

    pub async fn all_tasks(db: &impl ConnectionTrait) -> Result<Vec<Task>> {
        let models = task::Entity::find()
            .all(db)
            .await
            .context("Failed to get all tasks from database.")?;
        Ok(models.into_iter().map(|model| Self { model }).collect())
    }

    pub async fn complete_task(&mut self, db: &DatabaseConnection) -> Result<()> {
        if self.model.completed {
            return Ok(());
        }
        let task_id = self.model.id;
        let player_id = self.model.player_id;
        let mut new_task = self.model.clone();
        self.model = db
            .transaction::<_, Model, DbError>(|txn| {
                Box::pin(async move {
                    new_task.completed = true;
                    let active_model = ActiveModel {
                        completed: sea_orm::ActiveValue::Set(true),
                        ..new_task.clone().into_active_model()
                    };
                    task::Entity::update(active_model)
                        .exec(txn)
                        .await
                        .with_context(|| format!("Failed to update task with id {task_id}."))?;

                    let mut player = Player::from_id(txn, player_id)
                        .await
                        .with_context(|| format!("Failed to get owner of task {}", task_id))?;

                    player.add_xp(txn, 1.0).await.with_context(|| {
                        format!(
                        "Failed to add xp to player {player_id} while completing task {task_id}."
                    )
                    })?;
                    Ok(new_task)
                })
            })
            .await
            .context("Failed to complete transaction")?;
        Ok(())
    }
}
