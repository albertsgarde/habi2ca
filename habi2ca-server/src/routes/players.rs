use std::collections::HashMap;

use actix_web::{get, patch, post, web, HttpRequest, Responder, Scope};
use anyhow::Context;

use crate::{routes::RouteError, state::State};

#[post("")]
pub async fn create_player(
    state: web::Data<State>,
    query: web::Query<HashMap<String, String>>,
) -> Result<impl Responder, RouteError> {
    let player_name = query.get("name").context("Missing 'name' parameter")?;
    let player_id = state.database().create_player(player_name).await?;
    let player = state
        .database()
        .get_player(player_id)
        .await?
        .with_context(|| format!("Failed to get player with id {player_id} from database."))?;
    Ok(web::Json(player))
}

#[get("/{id}")]
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
        .with_context(|| format!("Failed to get player with id {player_id} from database."))?;
    Ok(web::Json(player))
}

#[patch("/{id}/add_xp")]
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
    let player = state
        .database()
        .get_player(player_id)
        .await?
        .with_context(|| format!("Failed to get player with id {player_id} from database."))?;
    Ok(web::Json(player))
}

pub fn add_routes(scope: Scope) -> Scope {
    scope
        .service(get_player)
        .service(create_player)
        .service(add_xp)
}

#[cfg(test)]
mod tests {
    use crate::{database::Database, start::create_app};

    use actix_web::test::{self, TestRequest};
    use habi2ca_common::player::{Player, PlayerId};

    #[tokio::test]
    async fn create_player() {
        let database = Database::create_in_memory().await.unwrap();
        let app = test::init_service(create_app(database)).await;

        let player: Player = test::call_and_read_body_json(
            &app,
            TestRequest::post()
                .uri("/api/players/?name=Alice")
                .to_request(),
        )
        .await;

        println!("{:?}", player);
        assert_eq!(player.id, PlayerId(1));
        assert_eq!(player.data.name, "Alice");
        assert_eq!(player.data.xp, 0.0);
    }

    #[tokio::test]
    async fn get_player() {
        let database = Database::create_in_memory().await.unwrap();
        database.create_player("Alice").await.unwrap();
        let app = test::init_service(create_app(database)).await;

        let resp: Player = test::call_and_read_body_json(
            &app,
            TestRequest::get().uri("/api/players/1").to_request(),
        )
        .await;

        assert_eq!(resp.id.0, 1);
        assert_eq!(resp.data.name, "Alice");
        assert_eq!(resp.data.xp, 0.0);
    }

    #[tokio::test]
    async fn add_xp() {
        let database = Database::create_in_memory().await.unwrap();
        database.create_player("Alice").await.unwrap();
        let app = test::init_service(create_app(database)).await;

        let add_xp_req = TestRequest::patch()
            .uri("/api/players/1/add_xp?xp=10.0")
            .to_request();

        let player: Player = test::call_and_read_body_json(
            &app,
            TestRequest::get().uri("/api/players/1").to_request(),
        )
        .await;

        assert_eq!(player.id.0, 1);
        assert_eq!(player.data.name, "Alice");
        assert_eq!(player.data.xp, 0.0);

        let player: Player = test::call_and_read_body_json(&app, add_xp_req).await;

        assert_eq!(player.id.0, 1);
        assert_eq!(player.data.name, "Alice");
        assert_eq!(player.data.xp, 10.0);
    }
}
