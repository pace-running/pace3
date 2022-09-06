use crate::models::event;
use actix_web::{web, Error, HttpResponse, Result};
use tera::Context;

pub async fn show(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    ctx.insert("event", &event::current_event());
    let rendered = tmpl.render("imprint.html", &ctx).unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}


#[cfg(test)]
mod tests {
    use crate::handlers::imprint::show;
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
    async fn imprint_page() {
        let tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(_e) => std::process::exit(1),
        };
        let data = actix_web::web::Data::new(tera);
        let resp = show(data).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let body = to_bytes(resp.into_body()).await.unwrap();
        let event = current_event();
        assert!(body.as_str().contains("Impressum"))
    }
}