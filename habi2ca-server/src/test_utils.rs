use actix_http::Request;
use actix_service::Service;
use actix_web::test as actix_test;
use habi2ca_database::migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use serde::de::DeserializeOwned;

pub async fn setup_database() -> DatabaseConnection {
    let database = Database::connect("sqlite::memory:")
        .await
        .expect("Failed to connect to database.");

    Migrator::up(&database, None)
        .await
        .expect("Failed to run migrations.");

    database
}

pub async fn assert_ok_response<S, E, R>(app: &S, req: Request) -> R
where
    S: Service<Request, Response = actix_web::dev::ServiceResponse, Error = E>,
    E: std::fmt::Debug,
    R: DeserializeOwned,
{
    let response = actix_test::call_service(app, req).await;
    if !response.status().is_success() {
        let body = actix_test::read_body(response).await;

        panic!("{}", std::str::from_utf8(&body).unwrap());
    } else {
        actix_test::read_body_json(response).await
    }
}

#[cfg(test)]
mod test {

    #[tokio::test]
    async fn setup_test_database() {
        let _ = super::setup_database().await;
    }
}
