use std::collections::HashMap;

use actix_web::{
    get, patch, post,
    web::{self, Json},
    HttpRequest, Responder, Scope,
};
use anyhow::Context;
use habi2ca_database::{
    player::PlayerId,
    task::{self, ActiveModel, TaskId},
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
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

#[patch("/{id}/complete")]
pub async fn complete_task(
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

    if task.completed {
        return Ok(web::Json(task));
    }

    println!("Completing task with id {task_id}.");
    let mut task: ActiveModel = task.into();
    task.completed = sea_orm::ActiveValue::Set(true);
    let task = task::Entity::update(task)
        .exec(state.database())
        .await
        .with_context(|| format!("Failed to update task with id {task_id}."))?;
    Ok(web::Json(task))
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
    }
}
