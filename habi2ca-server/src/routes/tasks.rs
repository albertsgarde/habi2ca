use std::{collections::HashMap, fmt::Display};

use actix_web::{
    get, patch, post,
    web::{self, Json},
    HttpRequest, Responder, Scope,
};
use anyhow::Context;
use habi2ca_database::{
    player::{self, PlayerId},
    task::{self, ActiveModel, TaskId},
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, TransactionTrait};
use serde::{Deserialize, Serialize};

use crate::{routes::RouteError, state::State};

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

#[post("")]
pub async fn create_task(
    state: web::Data<State>,
    task: Json<TaskData>,
) -> Result<impl Responder, RouteError> {
    let task_data = task.into_inner();
    let task = task::Entity::insert(task_data.into_active_model())
        .exec_with_returning(state.database())
        .await
        .context("Failed to insert task into database.")?;
    Ok(web::Json(task))
}

#[get("")]
pub async fn get_tasks(
    state: web::Data<State>,
    query: web::Query<HashMap<String, String>>,
) -> Result<impl Responder, RouteError> {
    let player_id = query
        .get("player")
        .map(|s| {
            s.parse()
                .with_context(|| format!("Failed to parse player id '{s}'."))
                .map(PlayerId)
        })
        .transpose()?;
    let selection = task::Entity::find();
    let selection = if let Some(player_id) = player_id {
        selection.filter(task::Column::PlayerId.eq(player_id))
    } else {
        selection
    };
    let tasks = selection
        .all(state.database())
        .await
        .context("Failed to get tasks.")?;
    Ok(web::Json(tasks))
}

#[get("/{id}")]
pub async fn get_task(
    state: web::Data<State>,
    request: HttpRequest,
) -> Result<impl Responder, RouteError> {
    let task_id: TaskId = request
        .match_info()
        .load()
        .context("Missing 'id' parameter")?;
    let task = task::Entity::find_by_id(task_id)
        .one(state.database())
        .await
        .with_context(|| format!("Failed to get task with id {task_id} from database."))?
        .with_context(|| format!("No task with id {task_id} exists."))?;
    Ok(web::Json(task))
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

#[patch("/{id}/complete")]
pub async fn complete_task(
    state: web::Data<State>,
    request: HttpRequest,
) -> Result<impl Responder, RouteError> {
    let task_id: TaskId = request
        .match_info()
        .load()
        .context("Missing 'id' parameter")?;

    println!("Completing task with id {task_id}.");

    Ok(state
        .database()
        .transaction::<_, web::Json<_>, DbError>(|txn| {
            Box::pin(async move {
                let task = task::Entity::find_by_id(task_id)
                    .one(txn)
                    .await
                    .with_context(|| {
                        format!("Failed to get task with id {task_id} from database.")
                    })?
                    .with_context(|| format!("No task with id {task_id} exists."))?;

                if task.completed {
                    return Ok(web::Json(task));
                }
                let mut task: ActiveModel = task.into();
                task.completed = sea_orm::ActiveValue::Set(true);
                let task = task::Entity::update(task)
                    .exec(txn)
                    .await
                    .with_context(|| format!("Failed to update task with id {task_id}."))?;

                let player = player::Entity::find_by_id(task.player_id)
                    .one(txn)
                    .await
                    .with_context(|| {
                        format!("Failed to get owner {} of task {}", task.player_id, task.id)
                    })?
                    .with_context(|| {
                        format!(
                            "Owner {} of task {} does not exist.",
                            task.player_id, task.id
                        )
                    })?;
                let prev_xp = player.xp;
                let player_id = task.player_id;
                let mut player: player::ActiveModel = player.into();
                player.xp = sea_orm::ActiveValue::Set(prev_xp + 1.0);
                let _player = player::Entity::update(player)
                    .exec(txn)
                    .await
                    .with_context(|| format!("Failed to update player with id {player_id}."))?;
                Ok(web::Json(task))
            })
        })
        .await
        .context("Failed to complete transaction")?)
}

pub fn add_routes(scope: Scope) -> Scope {
    scope
        .service(create_task)
        .service(get_tasks)
        .service(get_task)
        .service(complete_task)
}

#[cfg(test)]
mod tests {
    use actix_web::test::{self as actix_test, TestRequest};
    use habi2ca_database::{
        player::{self},
        prelude::Player,
        task,
    };
    use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait};

    use crate::{routes::tasks::TaskData, start::create_app, test};

    async fn setup_database() -> (DatabaseConnection, player::Model) {
        let database = test::setup_database().await;

        let player = Player::insert(player::ActiveModel {
            name: ActiveValue::Set("Alice".to_string()),
            xp: ActiveValue::Set(0.0),
            ..Default::default()
        })
        .exec_with_returning(&database)
        .await
        .unwrap();

        (database, player)
    }

    #[tokio::test]
    async fn create_task() {
        let (database, player) = setup_database().await;
        let app = actix_test::init_service(create_app(database)).await;

        let request = TestRequest::post()
            .uri("/api/tasks")
            .set_json(TaskData {
                player: player.id,
                name: "Task1".to_string(),
                description: "Description1".to_string(),
                completed: false,
            })
            .to_request();
        let task: task::Model = test::assert_ok_response(&app, request).await;
        println!("{task:?}");
        assert_eq!(task.id.0, 1);
        assert_eq!(task.player_id, player.id);
        assert_eq!(task.name, "Task1");
        assert_eq!(task.description, "Description1");
        assert_eq!(task.completed, false);
    }

    #[tokio::test]
    async fn get_tasks() {
        let (database, player) = setup_database().await;

        let task1 = task::ActiveModel {
            player_id: ActiveValue::Set(player.id),
            name: ActiveValue::Set("Task1".to_string()),
            description: ActiveValue::Set("Description1".to_string()),
            completed: ActiveValue::Set(false),
            ..Default::default()
        };

        let task1 = task::Entity::insert(task1)
            .exec_with_returning(&database)
            .await
            .unwrap();

        let task2 = task::ActiveModel {
            player_id: ActiveValue::Set(player.id),
            name: ActiveValue::Set("Task2".to_string()),
            description: ActiveValue::Set("Description2".to_string()),
            completed: ActiveValue::Set(true),
            ..Default::default()
        };

        let task2 = task::Entity::insert(task2)
            .exec_with_returning(&database)
            .await
            .unwrap();

        let app = actix_test::init_service(create_app(database)).await;

        let tasks: Vec<task::Model> =
            test::assert_ok_response(&app, TestRequest::get().uri("/api/tasks").to_request()).await;

        assert_eq!(tasks.len(), 2);
        assert_eq!(tasks[0].id, task1.id);
        assert_eq!(tasks[0].player_id, player.id);
        assert_eq!(tasks[0].name, "Task1");
        assert_eq!(tasks[0].description, "Description1");
        assert_eq!(tasks[0].completed, false);

        assert_eq!(tasks[1].id, task2.id);
        assert_eq!(tasks[1].player_id, player.id);
        assert_eq!(tasks[1].name, "Task2");
        assert_eq!(tasks[1].description, "Description2");
        assert_eq!(tasks[1].completed, true);
    }

    #[tokio::test]
    async fn get_player_tasks() {
        let (database, player1) = setup_database().await;

        let player2 = Player::insert(player::ActiveModel {
            name: ActiveValue::Set("Bob".to_string()),
            xp: ActiveValue::Set(0.0),
            ..Default::default()
        })
        .exec_with_returning(&database)
        .await
        .unwrap();

        let _task1 = task::Entity::insert(task::ActiveModel {
            player_id: ActiveValue::Set(player1.id),
            name: ActiveValue::Set("Task1".to_string()),
            description: ActiveValue::Set("Description1".to_string()),
            completed: ActiveValue::Set(false),
            ..Default::default()
        })
        .exec_with_returning(&database)
        .await
        .unwrap();

        let task2 = task::Entity::insert(task::ActiveModel {
            player_id: ActiveValue::Set(player2.id),
            name: ActiveValue::Set("Task2".to_string()),
            description: ActiveValue::Set("Description2".to_string()),
            completed: ActiveValue::Set(false),
            ..Default::default()
        })
        .exec_with_returning(&database)
        .await
        .unwrap();

        let app = actix_test::init_service(create_app(database)).await;

        let tasks: Vec<task::Model> = test::assert_ok_response(
            &app,
            TestRequest::get().uri("/api/tasks?player=2").to_request(),
        )
        .await;

        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].id, task2.id);
        assert_eq!(tasks[0].player_id, task2.player_id);
        assert_eq!(tasks[0].name, "Task2");
        assert_eq!(tasks[0].description, "Description2");
        assert_eq!(tasks[0].completed, false);
    }

    #[tokio::test]
    async fn get_task() {
        let (database, player) = setup_database().await;

        let _task = task::Entity::insert(task::ActiveModel {
            player_id: ActiveValue::Set(player.id),
            name: ActiveValue::Set("Task1".to_string()),
            description: ActiveValue::Set("Description1".to_string()),
            completed: ActiveValue::Set(false),
            ..Default::default()
        })
        .exec_with_returning(&database)
        .await
        .unwrap();

        let app = actix_test::init_service(create_app(database)).await;

        let response_task: task::Model =
            test::assert_ok_response(&app, TestRequest::get().uri("/api/tasks/1").to_request())
                .await;

        println!("{response_task:?}");
        assert_eq!(response_task.id.0, 1);
        assert_eq!(response_task.player_id, player.id);
        assert_eq!(response_task.name, "Task1");
        assert_eq!(response_task.description, "Description1");
        assert_eq!(response_task.completed, false);
    }

    #[tokio::test]
    async fn complete_task() {
        let (database, player) = setup_database().await;

        let _task = task::Entity::insert(task::ActiveModel {
            player_id: ActiveValue::Set(player.id),
            name: ActiveValue::Set("Task1".to_string()),
            description: ActiveValue::Set("Description1".to_string()),
            completed: ActiveValue::Set(false),
            ..Default::default()
        })
        .exec_with_returning(&database)
        .await
        .unwrap();

        let app = actix_test::init_service(create_app(database)).await;

        let response_task: task::Model = test::assert_ok_response(
            &app,
            TestRequest::patch()
                .uri("/api/tasks/1/complete")
                .to_request(),
        )
        .await;

        println!("{response_task:?}");
        assert_eq!(response_task.id.0, 1);
        assert_eq!(response_task.player_id, player.id);
        assert_eq!(response_task.name, "Task1");
        assert_eq!(response_task.description, "Description1");
        assert_eq!(response_task.completed, true);

        let response_player: player::Model =
            test::assert_ok_response(&app, TestRequest::get().uri("/api/players/1").to_request())
                .await;

        assert!(response_player.xp > player.xp);
        assert_eq!(response_player.name, player.name);
    }
}
