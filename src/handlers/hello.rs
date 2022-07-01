use actix_web::{Error, HttpResponse};
use std::path::PathBuf;
use actix_files::NamedFile;
use actix_web::{HttpRequest, Result};

pub async fn index(_req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = "./static/index.html".parse().unwrap();
    Ok(NamedFile::open(path)?) }

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
    async fn index() {
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