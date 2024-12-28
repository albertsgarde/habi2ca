use actix_web::{get, web, Responder, Scope};

use crate::{logic::level::Level, routes::RouteError, state::State};

#[get("")]
pub async fn get_levels(state: web::Data<State>) -> Result<impl Responder, RouteError> {
    let levels = Level::all_levels(state.database()).await?;
    Ok(web::Json(levels))
}

pub fn add_routes(scope: Scope) -> Scope {
    scope.service(get_levels)
}

#[cfg(test)]
mod tests {
    use actix_web::test::{self as actix_test, TestRequest};
    use habi2ca_database::level::{self};

    use crate::{start::create_app, test_utils};

    #[tokio::test]
    async fn get_levels() {
        let database = test_utils::setup_database().await;
        let app = actix_test::init_service(create_app(database)).await;

        let xp_requirements: Vec<f64> =
            serde_json::from_str(include_str!("../../../gamedata/levels.json"))
                .expect("Failed to parse levels.json");

        let all_levels: Vec<level::Model> = test_utils::assert_ok_response(
            &app,
            TestRequest::get().uri("/api/levels").to_request(),
        )
        .await;

        for (index, (xp, level)) in xp_requirements
            .into_iter()
            .zip(all_levels.into_iter())
            .enumerate()
        {
            assert_eq!(
                level.xp_requirement, xp,
                "XP requirement mismatch for level {index}",
            );
        }
    }
}
