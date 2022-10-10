use crate::models::event;
use actix_files::NamedFile;
use actix_web::{web, Error, HttpRequest, HttpResponse, Result};
use std::path::PathBuf;
use tera::Context;

pub async fn index(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let path: String = "index".to_string();
    result_template(tmpl, path).await
}

pub async fn template(
    tmpl: web::Data<tera::Tera>,
    page: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let path = page.into_inner();
    result_template(tmpl, path).await
}

pub async fn result_template(
    tmpl: web::Data<tera::Tera>,
    name: String,
) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    ctx.insert("event", &event::current_event());
    let rendered = tmpl.render(&(name + ".html"), &ctx).unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

pub async fn file(file: web::Path<String>, _req: HttpRequest) -> Result<NamedFile, Error> {
    let path: PathBuf = (String::from("./static/") + &file.into_inner())
        .parse()
        .unwrap();
    Ok(NamedFile::open(path)?)
}

#[cfg(test)]
mod tests {
    use crate::handlers::hello::index;
    use crate::models::event::current_event;
    use actix_web::body::to_bytes;
    use actix_web::http::StatusCode;
    use actix_web::web::Bytes;
    use tera::Tera;

    trait BodyTest {
        fn as_str(&self) -> &str;
    }

    impl BodyTest for Bytes {
        fn as_str(&self) -> &str {
            std::str::from_utf8(self).unwrap()
        }
    }

    #[actix_web::test]
    async fn unit_start_page() {
        let tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(_e) => std::process::exit(1),
        };
        let data = actix_web::web::Data::new(tera);
        let resp = index(data).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let body = to_bytes(resp.into_body()).await.unwrap();
        let event = current_event();
        assert!(body.as_str().contains(event.description.as_str()))
    }
}
