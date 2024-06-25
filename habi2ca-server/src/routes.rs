use actix_web::{get, post, web, HttpRequest, Responder, ResponseError};
use anyhow::{Context, Result};
use std::collections::HashMap;
use thiserror::Error;

use crate::state::State;

#[derive(Debug, Error)]
pub enum RouteError {
    #[error("{0:?}\n")]
    Anyhow(anyhow::Error),
}

impl From<anyhow::Error> for RouteError {
    fn from(error: anyhow::Error) -> Self {
        RouteError::Anyhow(error)
    }
}

impl ResponseError for RouteError {}

#[get("/player/{id}")]
pub async fn get_player(
    state: web::Data<State>,
    request: HttpRequest,
) -> Result<impl Responder, RouteError> {
    let player_id = request
        .match_info()
        .load()
        .context("Missing 'id' parameter")?;
    let player = state
        .database()
        .get_player(player_id)
        .await?
        .with_context(|| format!("No player with id {player_id} exists."))?;
    Ok(web::Json(player))
}

#[post("/players/create")]
pub async fn create_player(
    state: web::Data<State>,
    query: web::Query<HashMap<String, String>>,
) -> Result<impl Responder, RouteError> {
    let player_name = query.get("name").context("Missing 'name' parameter")?;
    println!("Creating player with name {player_name}.");
    let player_id = state.database().create_player(player_name).await?;
    Ok(web::Json(player_id))
}

#[post("/player/{id}/add_xp")]
pub async fn add_xp(
    state: web::Data<State>,
    request: HttpRequest,
    query: web::Query<HashMap<String, f32>>,
) -> Result<impl Responder, RouteError> {
    let player_id = request
        .match_info()
        .load()
        .context("Missing 'id' parameter")?;
    let &xp_delta = query.get("xp").context("Missing 'xp' parameter")?;
    println!("Adding {xp_delta} to player with id {player_id}.");
    state.database().add_xp(player_id, xp_delta).await?;
    Ok(web::Json(()))
}

#[cfg(test)]
mod tests {
    use crate::{database::Database, start::create_app};

    use actix_web::{
        http,
        test::{self, TestRequest},
    };
    use habi2ca_common::player::{Player, PlayerId};

    #[tokio::test]
    async fn test_get_player() {
        let database = Database::create_in_memory().await.unwrap();
        let app = test::init_service(create_app(database)).await;

        let create_player_req = TestRequest::post()
            .uri("/api/players/create?name=Alice")
            .to_request();
        let add_xp_req = TestRequest::post()
            .uri("/api/player/1/add_xp?xp=10.0")
            .to_request();

        let resp: PlayerId = test::call_and_read_body_json(&app, create_player_req).await;

        assert_eq!(resp.0, 1);

        let resp: Player = test::call_and_read_body_json(
            &app,
            TestRequest::get().uri("/api/player/1").to_request(),
        )
        .await;

        assert_eq!(resp.id.0, 1);
        assert_eq!(resp.data.name, "Alice");
        assert_eq!(resp.data.xp, 0.0);

        let resp = test::call_service(&app, add_xp_req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);

        let resp: Player = test::call_and_read_body_json(
            &app,
            TestRequest::get().uri("/api/player/1").to_request(),
        )
        .await;

        assert_eq!(resp.id.0, 1);
        assert_eq!(resp.data.name, "Alice");
        assert_eq!(resp.data.xp, 10.0);
    }
}
