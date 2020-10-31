mod routes;
use routes::landing::landing;
use routes::about::about;
use actix_web::{App, HttpServer, web};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(landing))
            .route("/about", web::get().to(about))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}