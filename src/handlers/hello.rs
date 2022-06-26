use actix_web::{Error, HttpResponse};

pub async fn hello_world() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("Hello World!"))
}

pub async fn hey() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("Hey There!"))
}

pub async fn echo(req_body: String ) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body(req_body))
}

#[cfg(test)]
mod tests {
    use crate::app_config::routes;
    use actix_web::dev::Service;
    use actix_web::{
        http::{StatusCode},
        test, App,
    };
    use actix_web::body::to_bytes;

    #[actix_web::test]
    async fn hello_world() {
        let app = test::init_service(App::new().configure(routes)).await;

        let req = test::TestRequest::get()
            .uri("/")
            .to_request();

        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);

        let body = to_bytes(resp.into_body()).await.unwrap();
        assert_eq!(body ,"Hello World!")
    }
    #[actix_web::test]
    async fn echo() {
        let app = test::init_service(App::new().configure(routes)).await;
        let test_string = "I EXPECT THIS STRING TO BE RETURNED IMMEDIATELY";

        let req = test::TestRequest::post()
            .uri("/echo")
            .set_payload(test_string)
            .to_request();

        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);

        let body = to_bytes(resp.into_body()).await.unwrap();
        assert_eq!(body, test_string)
    }
}