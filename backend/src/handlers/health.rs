use actix_web::{Error, HttpResponse};

pub async fn health() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("healthy"))
}
