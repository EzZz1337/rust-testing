use actix_web::{Error, HttpResponse};
use actix_web::http::{StatusCode};

pub async fn about() -> Result<HttpResponse, Error> {
    Ok(
        HttpResponse::build(StatusCode::OK)
            .content_type("text/html; charset=utf-8")
            .body(include_str!("../../pages/about/about.html"))
    )
}