use std::collections::HashMap;

use actix_web::{get, patch, post, web, HttpRequest, Responder, Scope};
use anyhow::Context;
use habi2ca_database::player::{self, PlayerId};
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait};

use crate::{routes::RouteError, state::State};

#[post("")]
pub async fn create_player(
    state: web::Data<State>,
    query: web::Query<HashMap<String, String>>,
) -> Result<impl Responder, RouteError> {
    let player_name = query.get("name").context("Missing 'name' parameter")?;
    let player = player::ActiveModel {
        name: ActiveValue::Set(player_name.clone()),
        xp: ActiveValue::Set(0.0),
        ..Default::default()
    };
    let player = player::Entity::insert(player)
        .exec_with_returning(state.database())
        .await
        .context("Failed to insert player into database.")?;

    Ok(web::Json(player))
}

#[get("/{id}")]
pub async fn get_player(
    state: web::Data<State>,
    request: HttpRequest,
) -> Result<impl Responder, RouteError> {
    let player_id: PlayerId = request
        .match_info()
        .load()
        .context("Missing 'id' parameter")?;
    let player = player::Entity::find_by_id(player_id)
        .one(state.database())
        .await
        .with_context(|| {
            format!("Failure while getting player with id {player_id} from database.")
        })?
        .with_context(|| format!("Player with id {player_id} not found in database."))?;
    Ok(web::Json(player))
}

#[patch("/{id}/add_xp")]
pub async fn add_xp(
    state: web::Data<State>,
    request: HttpRequest,
    query: web::Query<HashMap<String, f64>>,
) -> Result<impl Responder, RouteError> {
    let player_id: PlayerId = request
        .match_info()
        .load()
        .context("Missing 'id' parameter")?;
    let &xp_delta = query.get("xp").context("Missing 'xp' parameter")?;
    println!("Adding {xp_delta} to player with id {player_id}.");

    let player = player::Entity::find_by_id(player_id)
        .one(state.database())
        .await
        .with_context(|| {
            format!("Failure while getting player with id {player_id} from database.")
        })?
        .with_context(|| format!("Player with id {player_id} not found in database."))?;

    let new_xp = player.xp + xp_delta;

    let mut active_player: player::ActiveModel = player.into();
    active_player.xp = ActiveValue::Set(new_xp);

    let player = active_player
        .update(state.database())
        .await
        .with_context(|| {
            format!("Failure while updating player with id {player_id} in database.")
        })?;
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
    use crate::{start::create_app, test};

    use actix_web::test::{self as actix_test, TestRequest};
    use habi2ca_database::{
        player::{self, PlayerId},
        prelude::Player,
    };
    use sea_orm::{ActiveValue, EntityTrait};

    #[tokio::test]
    async fn create_player() {
        let database = test::setup_database().await;
        let app = actix_test::init_service(create_app(database)).await;

        let player: player::Model = test::assert_ok_response(
            &app,
            TestRequest::post()
                .uri("/api/players/?name=Alice")
                .to_request(),
        )
        .await;

        println!("{:?}", player);
        assert_eq!(player.id, PlayerId(1));
        assert_eq!(player.name, "Alice");
        assert_eq!(player.xp, 0.0);
    }

    #[tokio::test]
    async fn get_player() {
        let database = test::setup_database().await;
        let player = player::ActiveModel {
            name: ActiveValue::Set("Alice".to_string()),
            xp: ActiveValue::Set(0.0),
            ..Default::default()
        };
        let _player = Player::insert(player)
            .exec_with_returning(&database)
            .await
            .unwrap();

        let app = actix_test::init_service(create_app(database)).await;

        let resp: player::Model =
            test::assert_ok_response(&app, TestRequest::get().uri("/api/players/1").to_request())
                .await;

        assert_eq!(resp.id.0, 1);
        assert_eq!(resp.name, "Alice");
        assert_eq!(resp.xp, 0.0);
    }

    #[tokio::test]
    async fn add_xp() {
        let database = test::setup_database().await;
        let player = player::ActiveModel {
            name: ActiveValue::Set("Alice".to_string()),
            xp: ActiveValue::Set(0.0),
            ..Default::default()
        };
        let _player = Player::insert(player)
            .exec_with_returning(&database)
            .await
            .unwrap();

        let app = actix_test::init_service(create_app(database)).await;

        let add_xp_req = TestRequest::patch()
            .uri("/api/players/1/add_xp?xp=10.0")
            .to_request();

        let player: player::Model =
            test::assert_ok_response(&app, TestRequest::get().uri("/api/players/1").to_request())
                .await;

        assert_eq!(player.id.0, 1);
        assert_eq!(player.name, "Alice");
        assert_eq!(player.xp, 0.0);

        let player: player::Model = test::assert_ok_response(&app, add_xp_req).await;

        assert_eq!(player.id.0, 1);
        assert_eq!(player.name, "Alice");
        assert_eq!(player.xp, 10.0);
    }
}
