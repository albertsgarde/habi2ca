use anyhow::bail;
use anyhow::Context;
use anyhow::Result;
use habi2ca_common::player::PlayerId;
use habi2ca_common::task::{Task, TaskData, TaskId};
use tokio_rusqlite::OptionalExtension;

use super::Database;

impl Database {
    pub async fn create_task(&self, task: TaskData) -> Result<TaskId> {
        const CREATE_TASK: &str =
            "INSERT INTO task (player_id, name, description, completed) VALUES (?1, ?2, ?3, ?4)";

        self.connection
            .call_unwrap(move |connection| {
                let mut statement = connection.prepare(CREATE_TASK)?;

                statement
                    .insert((
                        task.player,
                        task.name.as_str(),
                        task.description.as_str(),
                        task.completed,
                    ))
                    .with_context(move || format!("Failed to insert row for new task {task:?}."))
                    .map(TaskId)
            })
            .await
    }

    pub async fn get_tasks(&self) -> Result<Vec<Task>> {
        const GET_TASKS: &str = "SELECT id, player_id, name, description, completed FROM task";

        self.connection
            .call_unwrap(move |connection| {
                let mut statement = connection.prepare(GET_TASKS)?;
                let tasks = statement
                    .query_map((), |row| {
                        Ok(Task {
                            id: row.get(0)?,
                            data: TaskData {
                                player: row.get(1)?,
                                name: row.get(2)?,
                                description: row.get(3)?,
                                completed: row.get(4)?,
                            },
                        })
                    })
                    .context("SQLite call to get tasks failed.")?
                    .collect::<Result<_, _>>()
                    .context("Failed to construct task from DB row.")?;
                Ok(tasks)
            })
            .await
    }

    pub async fn get_player_tasks(&self, player_id: PlayerId) -> Result<Vec<Task>> {
        const GET_PLAYER_TASKS: &str =
            "SELECT id, name, description, completed FROM task WHERE player_id = ?1";

        self.connection
            .call_unwrap(move |connection| {
                let mut statement = connection.prepare(GET_PLAYER_TASKS)?;
                let tasks = statement
                    .query_map((player_id,), |row| {
                        Ok(Task {
                            id: row.get(0)?,
                            data: TaskData {
                                player: player_id,
                                name: row.get(1)?,
                                description: row.get(2)?,
                                completed: row.get(3)?,
                            },
                        })
                    })
                    .context("SQLite call to get tasks failed.")?
                    .collect::<Result<_, _>>()
                    .context("Failed to construct task from DB row.")?;
                Ok(tasks)
            })
            .await
    }

    pub async fn get_task(&self, task_id: TaskId) -> Result<Option<Task>> {
        const GET_TASK: &str =
            "SELECT player_id, name, description, completed FROM task WHERE id = ?1";

        self.connection
            .call_unwrap(move |connection| {
                let mut statement = connection.prepare(GET_TASK)?;
                statement
                    .query_row((task_id,), |row| {
                        Ok(Task {
                            id: task_id,
                            data: TaskData {
                                player: row.get(0)?,
                                name: row.get(1)?,
                                description: row.get(2)?,
                                completed: row.get(3)?,
                            },
                        })
                    })
                    .optional()
                    .with_context(|| format!("SQLite call to get task with id {task_id} failed."))
            })
            .await
    }

    pub async fn complete_task(&self, task_id: TaskId) -> Result<()> {
        const COMPLETE_TASK: &str = "UPDATE task SET completed = TRUE WHERE id = ?1";

        self.connection
            .call_unwrap(move |connection| {
                let mut statement = connection.prepare(COMPLETE_TASK)?;
                let num_rows_changed = statement
                    .execute((task_id,))
                    .with_context(|| format!("Failed to complete task with id {task_id}.",))?;
                match num_rows_changed {
                    1 => Ok(()),
                    0 => bail!("No task with id {task_id} exists."),
                    _ => bail!(
                        "Expected 1 row to be changed, but {num_rows_changed} rows were changed."
                    ),
                }
            })
            .await
    }
}
