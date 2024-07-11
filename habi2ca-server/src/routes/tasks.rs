use std::collections::HashMap;

use actix_web::{
    get, patch, post,
    web::{self, Json},
    HttpRequest, Responder, Scope,
};
use anyhow::{anyhow, Context};
use habi2ca_common::{player::PlayerId, task::TaskData};

use crate::{routes::RouteError, state::State};

#[post("")]
pub async fn create_task(
    state: web::Data<State>,
    task: Json<TaskData>,
) -> Result<impl Responder, RouteError> {
    let task_data = task.into_inner();
    let task_id = state.database().create_task(task_data.clone()).await?;
    let task = state
        .database()
        .get_task(task_id)
        .await?
        .context("Failed to get newly created task.")?;
    if task.data != task_data {
        Err(anyhow!(
            "Task data mismatch: {:?} != {task_data:?}.",
            task.data
        ))?;
    }
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
    if let Some(player_id) = player_id {
        let tasks = state
            .database()
            .get_player_tasks(player_id)
            .await
            .with_context(|| format!("Failed to get tasks for player {player_id}."))?;
        Ok(web::Json(tasks))
    } else {
        let tasks = state
            .database()
            .get_tasks()
            .await
            .context("Failed to get tasks.")?;
        Ok(web::Json(tasks))
    }
}

#[get("/{id}")]
pub async fn get_task(
    state: web::Data<State>,
    request: HttpRequest,
) -> Result<impl Responder, RouteError> {
    let task_id = request
        .match_info()
        .load()
        .context("Missing 'id' parameter")?;
    let task = state
        .database()
        .get_task(task_id)
        .await?
        .with_context(|| format!("No player with id {task_id} exists."))?;
    Ok(web::Json(task))
}

#[patch("/{id}/complete")]
pub async fn complete_task(
    state: web::Data<State>,
    request: HttpRequest,
) -> Result<impl Responder, RouteError> {
    let task_id = request
        .match_info()
        .load()
        .context("Missing 'id' parameter")?;
    let task = state
        .database()
        .get_task(task_id)
        .await?
        .with_context(|| format!("Failed to get task with id {task_id} from data base."))?;
    if task.data.completed() {
        return Ok(web::Json(task));
    }
    println!("Completing task with id {task_id}.");
    state.database().complete_task(task_id).await?;
    let task = state
        .database()
        .get_task(task_id)
        .await?
        .with_context(|| format!("Failed to get task with id {task_id} from data base."))?;
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
    use actix_web::test::{self, TestRequest};
    use habi2ca_common::task::{Task, TaskData};

    use crate::{database::Database, start::create_app};

    #[tokio::test]
    async fn get_tasks() {
        let database = Database::create_in_memory().await.unwrap();
        let player_id = database.create_player("Alice").await.unwrap();
        let task_id = database
            .create_task(TaskData::new(player_id, "Task1", "Description1"))
            .await
            .unwrap();
        let task_id2 = database
            .create_task(TaskData::new(player_id, "Task2", "Description2"))
            .await
            .unwrap();

        let app = test::init_service(create_app(database)).await;

        let tasks: Vec<Task> =
            test::call_and_read_body_json(&app, TestRequest::get().uri("/api/tasks").to_request())
                .await;

        assert_eq!(tasks.len(), 2);
        assert_eq!(tasks[0].id, task_id);
        assert_eq!(tasks[0].data.player, player_id);
        assert_eq!(tasks[0].data.name, "Task1");
        assert_eq!(tasks[0].data.description, "Description1");
        assert_eq!(tasks[0].data.completed, false);

        assert_eq!(tasks[1].id, task_id2);
        assert_eq!(tasks[1].data.player, player_id);
        assert_eq!(tasks[1].data.name, "Task2");
        assert_eq!(tasks[1].data.description, "Description2");
        assert_eq!(tasks[1].data.completed, false);
    }

    #[tokio::test]
    async fn get_player_tasks() {
        let database = Database::create_in_memory().await.unwrap();
        let player_id = database.create_player("Alice").await.unwrap();
        let player2_id = database.create_player("Bob").await.unwrap();
        let _task_id = database
            .create_task(TaskData::new(player2_id, "Task1", "Description1"))
            .await
            .unwrap();
        let task_id2 = database
            .create_task(TaskData::new(player_id, "Task2", "Description2"))
            .await
            .unwrap();

        let app = test::init_service(create_app(database)).await;

        let tasks: Vec<Task> = test::call_and_read_body_json(
            &app,
            TestRequest::get().uri("/api/tasks?player=1").to_request(),
        )
        .await;

        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].id, task_id2);
        assert_eq!(tasks[0].data.player, player_id);
        assert_eq!(tasks[0].data.name, "Task2");
        assert_eq!(tasks[0].data.description, "Description2");
        assert_eq!(tasks[0].data.completed, false);
    }

    #[tokio::test]
    async fn create_task() {
        let database = Database::create_in_memory().await.unwrap();
        let player_id = database.create_player("Alice").await.unwrap();
        let app = test::init_service(create_app(database)).await;

        let task: Task = test::call_and_read_body_json(
            &app,
            TestRequest::post()
                .uri("/api/tasks")
                .set_json(TaskData::new(player_id, "Task1", "Description1"))
                .to_request(),
        )
        .await;

        println!("{task:?}");
        assert_eq!(task.id.0, 1);
        assert_eq!(task.data.player, player_id);
        assert_eq!(task.data.name, "Task1");
        assert_eq!(task.data.description, "Description1");
        assert_eq!(task.data.completed, false);
    }

    #[tokio::test]
    async fn get_task() {
        let database = Database::create_in_memory().await.unwrap();
        let player_id = database.create_player("Alice").await.unwrap();
        let app = test::init_service(create_app(database)).await;

        let task: Task = test::call_and_read_body_json(
            &app,
            TestRequest::post()
                .uri("/api/tasks")
                .set_json(TaskData::new(player_id, "Task1", "Description1"))
                .to_request(),
        )
        .await;

        println!("{task:?}");
        assert_eq!(task.id.0, 1);
        assert_eq!(task.data.player, player_id);
        assert_eq!(task.data.name, "Task1");
        assert_eq!(task.data.description, "Description1");
        assert_eq!(task.data.completed, false);

        let resp: Task = test::call_and_read_body_json(
            &app,
            TestRequest::get().uri("/api/tasks/1").to_request(),
        )
        .await;

        assert_eq!(resp.id, task.id);
        assert_eq!(resp.data.player, player_id);
        assert_eq!(resp.data.name, "Task1");
        assert_eq!(resp.data.description, "Description1");
        assert_eq!(resp.data.completed, false);
    }

    #[tokio::test]
    async fn complete_task() {
        let database = Database::create_in_memory().await.unwrap();
        let player_id = database.create_player("Alice").await.unwrap();
        let app = test::init_service(create_app(database)).await;

        let task: Task = test::call_and_read_body_json(
            &app,
            TestRequest::post()
                .uri("/api/tasks")
                .set_json(TaskData::new(player_id, "Task1", "Description1"))
                .to_request(),
        )
        .await;

        println!("{task:?}");
        assert_eq!(task.id.0, 1);
        assert_eq!(task.data.player, player_id);
        assert_eq!(task.data.name, "Task1");
        assert_eq!(task.data.description, "Description1");
        assert_eq!(task.data.completed, false);

        let resp: Task = test::call_and_read_body_json(
            &app,
            TestRequest::patch()
                .uri("/api/tasks/1/complete")
                .to_request(),
        )
        .await;

        assert_eq!(resp.id.0, 1);
        assert_eq!(resp.data.player, player_id);
        assert_eq!(resp.data.name, "Task1");
        assert_eq!(resp.data.description, "Description1");
        assert_eq!(resp.data.completed, true);
    }
}
