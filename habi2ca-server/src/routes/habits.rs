use actix_web::{
    get, patch, post,
    web::{self, Json},
    HttpRequest, Responder, Scope,
};
use anyhow::Context;

use crate::{
    logic::habit::{Habit, HabitData},
    routes::RouteError,
    state::State,
};

#[post("")]
pub async fn create_habit(
    state: web::Data<State>,
    habit: Json<HabitData>,
) -> Result<impl Responder, RouteError> {
    let habit = Habit::create(state.database(), habit.into_inner())
        .await
        .context("Failed to create habit.")?;
    Ok(web::Json(habit))
}

#[get("")]
pub async fn get_habits(
    state: web::Data<State>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> Result<impl Responder, RouteError> {
    let player_id = query
        .get("player")
        .map(|s| {
            s.parse()
                .with_context(|| format!("Failed to parse player id '{s}'."))
                .map(habi2ca_database::player::PlayerId)
        })
        .transpose()?;

    let result = if let Some(player_id) = player_id {
        Habit::player_habits(state.database(), player_id).await?
    } else {
        Habit::all_habits(state.database()).await?
    };
    Ok(web::Json(result))
}

#[get("/{id}")]
pub async fn get_habit(
    state: web::Data<State>,
    request: HttpRequest,
) -> Result<impl Responder, RouteError> {
    let habit_id = request
        .match_info()
        .load()
        .context("Missing 'id' parameter")?;
    let habit = Habit::from_id(state.database(), habit_id)
        .await
        .with_context(|| format!("Failed to get habit with id {habit_id}."))?;
    Ok(web::Json(habit))
}

#[patch("/{id}/increment")]
pub async fn increment_habit(
    state: web::Data<State>,
    request: HttpRequest,
) -> Result<impl Responder, RouteError> {
    let habit_id = request
        .match_info()
        .load()
        .context("Missing 'id' parameter")?;
    let mut habit = Habit::from_id(state.database(), habit_id)
        .await
        .with_context(|| format!("Failed to get habit with id {habit_id}."))?;
    habit
        .increment(state.database())
        .await
        .with_context(|| format!("Failed to increment habit with id {habit_id}."))?;
    Ok(web::Json(habit))
}

pub fn add_routes(scope: Scope) -> Scope {
    scope
        .service(create_habit)
        .service(get_habits)
        .service(get_habit)
        .service(increment_habit)
}

#[cfg(test)]
mod test {
    use actix_web::test::{self as actix_test, TestRequest};
    use habi2ca_database::habit::HabitId;
    use sea_orm::DatabaseConnection;

    use crate::{
        logic::{
            habit::{Habit, HabitData},
            player::Player,
        },
        start::create_app,
        test_utils,
    };

    async fn setup_database() -> (DatabaseConnection, Player) {
        let database = test_utils::setup_database().await;

        let player = Player::create(&database, "Alice").await.unwrap();

        (database, player)
    }

    #[tokio::test]
    async fn create_habit() {
        let (database, player) = setup_database().await;
        let app = actix_test::init_service(create_app(database)).await;

        let request = TestRequest::post()
            .uri("/api/habits")
            .set_json(HabitData {
                player_id: player.id(),
                name: "Habit1".to_string(),
                description: "Description1".to_string(),
            })
            .to_request();

        let habit: Habit = test_utils::assert_ok_response(&app, request).await;

        assert_eq!(habit.id(), HabitId(1));
        assert_eq!(habit.player(), player.id());
        assert_eq!(habit.name(), "Habit1");
        assert_eq!(habit.description(), "Description1");
    }

    #[tokio::test]
    async fn get_habits() {
        let (database, player) = setup_database().await;

        let habit1 = Habit::create(
            &database,
            HabitData {
                player_id: player.id(),
                name: "Habit1".to_string(),
                description: "Description1".to_string(),
            },
        )
        .await
        .unwrap();

        let habit2 = Habit::create(
            &database,
            HabitData {
                player_id: player.id(),
                name: "Habit2".to_string(),
                description: "Description2".to_string(),
            },
        )
        .await
        .unwrap();

        let app = actix_test::init_service(create_app(database)).await;

        let habits: Vec<Habit> = test_utils::assert_ok_response(
            &app,
            TestRequest::get().uri("/api/habits").to_request(),
        )
        .await;

        assert_eq!(habits.len(), 2);

        assert_eq!(habits[0].id(), habit1.id());
        assert_eq!(habits[0].player(), player.id());
        assert_eq!(habits[0].name(), "Habit1");
        assert_eq!(habits[0].description(), "Description1");

        assert_eq!(habits[1].id(), habit2.id());
        assert_eq!(habits[1].player(), player.id());
        assert_eq!(habits[1].name(), "Habit2");
        assert_eq!(habits[1].description(), "Description2");
    }

    #[tokio::test]
    async fn get_player_habits() {
        let (database, player) = setup_database().await;

        let player2 = Player::create(&database, "Bob").await.unwrap();

        let _habit1 = Habit::create(
            &database,
            HabitData {
                player_id: player.id(),
                name: "Habit1".to_string(),
                description: "Description1".to_string(),
            },
        )
        .await
        .unwrap();

        let habit2 = Habit::create(
            &database,
            HabitData {
                player_id: player2.id(),
                name: "Habit2".to_string(),
                description: "Description2".to_string(),
            },
        )
        .await
        .unwrap();

        let app = actix_test::init_service(create_app(database)).await;

        let habits: Vec<Habit> = test_utils::assert_ok_response(
            &app,
            TestRequest::get().uri("/api/habits?player=2").to_request(),
        )
        .await;

        assert_eq!(habits.len(), 1);

        assert_eq!(habits[0].id(), habit2.id());
        assert_eq!(habits[0].player(), player2.id());
        assert_eq!(habits[0].name(), "Habit2");
        assert_eq!(habits[0].description(), "Description2");
    }

    #[tokio::test]
    async fn get_habit() {
        let (database, player) = setup_database().await;

        let habit1 = Habit::create(
            &database,
            HabitData {
                player_id: player.id(),
                name: "Habit1".to_string(),
                description: "Description1".to_string(),
            },
        )
        .await
        .unwrap();

        let app = actix_test::init_service(create_app(database)).await;

        let habit: Habit = test_utils::assert_ok_response(
            &app,
            TestRequest::get().uri("/api/habits/1").to_request(),
        )
        .await;

        assert_eq!(habit.id(), habit1.id());
        assert_eq!(habit.player(), player.id());
        assert_eq!(habit.name(), "Habit1");
        assert_eq!(habit.description(), "Description1");
    }

    #[tokio::test]
    async fn increment_habit() {
        let (database, player) = setup_database().await;

        let habit1 = Habit::create(
            &database,
            HabitData {
                player_id: player.id(),
                name: "Habit1".to_string(),
                description: "Description1".to_string(),
            },
        )
        .await
        .unwrap();

        let app = actix_test::init_service(create_app(database.clone())).await;

        let pre_xp = Player::from_id(&database, player.id()).await.unwrap().xp();

        let habit: Habit = test_utils::assert_ok_response(
            &app,
            TestRequest::patch()
                .uri("/api/habits/1/increment")
                .to_request(),
        )
        .await;

        let post_xp = Player::from_id(&database, player.id()).await.unwrap().xp();

        assert!(post_xp > pre_xp);

        assert_eq!(habit.id(), habit1.id());
        assert_eq!(habit.player(), player.id());
        assert_eq!(habit.name(), "Habit1");
        assert_eq!(habit.description(), "Description1");
    }
}
