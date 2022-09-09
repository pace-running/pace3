use actix_web::{Error, HttpResponse};

pub async fn health_endpoint() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().finish())
}

#[cfg(test)]
mod tests {
    use crate::app_config::routes;
    use actix_web::dev::Service;
    use actix_web::{http::StatusCode, test, App};

    #[actix_web::test]
    async fn unit_health() {
        let app = test::init_service(App::new().configure(routes)).await;

        let req = test::TestRequest::get().uri("/health").to_request();

        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
    }
}
