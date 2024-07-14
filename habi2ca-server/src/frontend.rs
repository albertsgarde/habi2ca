use actix_files::Files;
use actix_web::Scope;

pub fn add_routes(scope: Scope) -> Scope {
    scope.service(Files::new("/", "habi2ca-frontend/dist").index_file("index.html"))
}
