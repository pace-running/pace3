use actix_web::{Error, error, HttpRequest, HttpResponse, Result, web};
use std::path::PathBuf;
use actix_files::NamedFile;
use tera::Context;

pub async fn index(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let path: String = "index".to_string();
    result_template(tmpl, path).await
}

pub async fn template(tmpl: web::Data<tera::Tera>, page: web::Path<String>) -> Result<HttpResponse, Error> {
    let path = page.into_inner();
    result_template(tmpl, path).await
}

pub async fn result_template(tmpl: web::Data<tera::Tera>, name: String) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    ctx.insert("var", "Hello World");
    let rendered = tmpl.render(&(name + ".html"), &ctx).map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

pub async fn file(file: web::Path<String>, _req: HttpRequest) -> Result<NamedFile, Error> {
    let path: PathBuf = (String::from("./static/") + &file.into_inner()).parse().unwrap();
    Ok(NamedFile::open(path)?)
}

pub async fn hey() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("Hey There!"))
}
pub async fn echo(req_body: String ) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body(req_body))
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;
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

        let mut index_as_string = String::new();
        let mut f = File::open("../../templates/index.html").expect("Unable to read file");
        f.read_to_string(&mut index_as_string).expect("Unable to read string");

        let body = to_bytes(resp.into_body()).await.unwrap();
        assert_eq!(body ,index_as_string)
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