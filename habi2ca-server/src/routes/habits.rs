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
                player: player.id(),
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
                player: player.id(),
                name: "Habit1".to_string(),
                description: "Description1".to_string(),
            },
        )
        .await
        .unwrap();

        let habit2 = Habit::create(
            &database,
            HabitData {
                player: player.id(),
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
                player: player.id(),
                name: "Habit1".to_string(),
                description: "Description1".to_string(),
            },
        )
        .await
        .unwrap();

        let habit2 = Habit::create(
            &database,
            HabitData {
                player: player2.id(),
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
                player: player.id(),
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
                player: player.id(),
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
