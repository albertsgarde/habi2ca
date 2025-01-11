use std::collections::HashMap;

use actix_web::{
    get, patch, post,
    web::{self, Json},
    HttpRequest, Responder, Scope,
};
use anyhow::Context;
use habi2ca_database::{player::PlayerId, task::TaskId};

use crate::{
    logic::task::{Task, TaskData},
    routes::RouteError,
    state::State,
};

#[post("")]
pub async fn create_task(
    state: web::Data<State>,
    task: Json<TaskData>,
) -> Result<impl Responder, RouteError> {
    let task = Task::create(state.database(), task.into_inner()).await?;
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

    let result = if let Some(player_id) = player_id {
        Task::player_tasks(state.database(), player_id).await?
    } else {
        Task::all_tasks(state.database()).await?
    };
    Ok(web::Json(result))
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

    let task = Task::from_id(state.database(), task_id).await?;
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

    let mut task = Task::from_id(state.database(), task_id).await?;

    task.complete_task(state.database()).await?;
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
    use habi2ca_database::{level::LevelId, player, task};
    use sea_orm::DatabaseConnection;

    use crate::{
        logic::{level::Level, player::Player, task::Task},
        routes::tasks::TaskData,
        start::create_app,
        test_utils,
    };

    async fn setup_database() -> (DatabaseConnection, Player) {
        let database = test_utils::setup_database().await;

        let player = Player::create(&database, "Alice").await.unwrap();

        (database, player)
    }

    #[tokio::test]
    async fn create_task() {
        let (database, player) = setup_database().await;
        let app = actix_test::init_service(create_app(database)).await;

        let request = TestRequest::post()
            .uri("/api/tasks")
            .set_json(TaskData {
                player: player.id(),
                name: "Task1".to_string(),
                description: "Description1".to_string(),
                completed: false,
            })
            .to_request();
        let task: task::Model = test_utils::assert_ok_response(&app, request).await;
        assert_eq!(task.id.0, 1);
        assert_eq!(task.player_id, player.id());
        assert_eq!(task.name, "Task1");
        assert_eq!(task.description, "Description1");
        assert_eq!(task.completed, false);
    }

    #[tokio::test]
    async fn get_tasks() {
        let (database, player) = setup_database().await;

        let task1 = Task::create(
            &database,
            TaskData {
                player: player.id(),
                name: "Task1".to_string(),
                description: "Description1".to_string(),
                completed: false,
            },
        )
        .await
        .unwrap();

        let task2 = Task::create(
            &database,
            TaskData {
                player: player.id(),
                name: "Task2".to_string(),
                description: "Description2".to_string(),
                completed: true,
            },
        )
        .await
        .unwrap();

        let app = actix_test::init_service(create_app(database)).await;

        let tasks: Vec<task::Model> =
            test_utils::assert_ok_response(&app, TestRequest::get().uri("/api/tasks").to_request())
                .await;

        assert_eq!(tasks.len(), 2);

        assert_eq!(tasks[0].id, task1.id());
        assert_eq!(tasks[0].player_id, player.id());
        assert_eq!(tasks[0].name, "Task1");
        assert_eq!(tasks[0].description, "Description1");
        assert_eq!(tasks[0].completed, false);

        assert_eq!(tasks[1].id, task2.id());
        assert_eq!(tasks[1].player_id, player.id());
        assert_eq!(tasks[1].name, "Task2");
        assert_eq!(tasks[1].description, "Description2");
        assert_eq!(tasks[1].completed, true);
    }

    #[tokio::test]
    async fn get_player_tasks() {
        let (database, player1) = setup_database().await;

        let player2 = Player::create(&database, "Bob").await.unwrap();

        Task::create(
            &database,
            TaskData {
                player: player1.id(),
                name: "Task1".to_string(),
                description: "Description1".to_string(),
                completed: false,
            },
        )
        .await
        .unwrap();

        let task2 = Task::create(
            &database,
            TaskData {
                player: player2.id(),
                name: "Task2".to_string(),
                description: "Description2".to_string(),
                completed: false,
            },
        )
        .await
        .unwrap();

        let app = actix_test::init_service(create_app(database)).await;

        let tasks: Vec<task::Model> = test_utils::assert_ok_response(
            &app,
            TestRequest::get().uri("/api/tasks?player=2").to_request(),
        )
        .await;

        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].id, task2.id());
        assert_eq!(tasks[0].player_id, task2.player_id());
        assert_eq!(tasks[0].name, "Task2");
        assert_eq!(tasks[0].description, "Description2");
        assert_eq!(tasks[0].completed, false);
    }

    #[tokio::test]
    async fn get_task() {
        let (database, player) = setup_database().await;

        Task::create(
            &database,
            TaskData {
                player: player.id(),
                name: "Task1".to_string(),
                description: "Description1".to_string(),
                completed: false,
            },
        )
        .await
        .unwrap();

        let app = actix_test::init_service(create_app(database)).await;

        let response_task: task::Model = test_utils::assert_ok_response(
            &app,
            TestRequest::get().uri("/api/tasks/1").to_request(),
        )
        .await;

        assert_eq!(response_task.id.0, 1);
        assert_eq!(response_task.player_id, player.id());
        assert_eq!(response_task.name, "Task1");
        assert_eq!(response_task.description, "Description1");
        assert_eq!(response_task.completed, false);
    }

    #[tokio::test]
    async fn complete_task() {
        let (database, mut player) = setup_database().await;

        let level_1_xp = Level::from_id(&database, LevelId(1))
            .await
            .unwrap()
            .xp_requirement();

        player.add_xp(&database, level_1_xp - 0.5).await.unwrap();

        let task = Task::create(
            &database,
            TaskData {
                player: player.id(),
                name: "Task1".to_string(),
                description: "Description1".to_string(),
                completed: false,
            },
        )
        .await
        .unwrap();

        let app = actix_test::init_service(create_app(database)).await;

        let response_task: task::Model = test_utils::assert_ok_response(
            &app,
            TestRequest::patch()
                .uri(&format!("/api/tasks/{}/complete", task.id()))
                .to_request(),
        )
        .await;

        assert_eq!(response_task.id.0, 1);
        assert_eq!(response_task.player_id, player.id());
        assert_eq!(response_task.name, "Task1");
        assert_eq!(response_task.description, "Description1");
        assert_eq!(response_task.completed, true);

        let response_task: task::Model = test_utils::assert_ok_response(
            &app,
            TestRequest::get().uri("/api/tasks/1").to_request(),
        )
        .await;

        assert_eq!(response_task.id.0, 1);
        assert_eq!(response_task.player_id, player.id());
        assert_eq!(response_task.name, "Task1");
        assert_eq!(response_task.description, "Description1");
        assert_eq!(response_task.completed, true);

        let response_player: player::Model = test_utils::assert_ok_response(
            &app,
            TestRequest::get()
                .uri(&format!("/api/players/{}", task.player_id()))
                .to_request(),
        )
        .await;

        assert_eq!(response_player.level_id, LevelId(2));
        assert_eq!(response_player.xp, 0.5);
    }
}
