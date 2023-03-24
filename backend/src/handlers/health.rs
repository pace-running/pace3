use actix_web::{Error, HttpResponse};

pub async fn health() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("healthy"))
}


#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{
        body::to_bytes,
        http::{self}
    };

    #[actix_web::test]
    async fn test_health_ok() {
        let resp = health().await.unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);

        let body = to_bytes(resp.into_body()).await.unwrap();
        assert_eq!(body, "healthy");
    }
}