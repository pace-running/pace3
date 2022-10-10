use actix_web::{web, Error, HttpResponse, Result};
use tera::Context;

use crate::models::{event, info::Info};

use super::join::has_bad_data;

pub async fn show(
    form: web::Form<Info>,
    tmpl: web::Data<tera::Tera>,
) -> Result<HttpResponse, Error> {
    let mut ctx = Context::new();
    if has_bad_data(&form) {
        return Ok(HttpResponse::BadRequest().body("Bad data"));
    }
    ctx.insert("event", &event::current_event());
    ctx.insert("info", &form);
    let rendered = tmpl.render("summary.html", &ctx).unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

#[cfg(test)]
mod tests {
    use actix_web::body::to_bytes;
    use actix_web::{http::StatusCode, web, web::Bytes};
    use tera::Tera;

    use crate::builders::InfoBuilder;
    use crate::handlers::summary::show;
    use crate::models::info::{Info, RunnerInfo};

    trait BodyTest {
        fn as_str(&self) -> &str;
    }

    impl BodyTest for Bytes {
        fn as_str(&self) -> &str {
            std::str::from_utf8(self).unwrap()
        }
    }

    #[actix_web::test]
    async fn unit_summary_page_with_correct_form_data() {
        let tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(_e) => std::process::exit(1),
        };
        let data = web::Data::new(tera);
        let participant = InfoBuilder::default().build();
        let form = web::Form(participant);
        let response = show(form, data).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = to_bytes(response.into_body()).await.unwrap();
        assert!(body.as_str().contains("<h1>Zusammenfassung</h1>"))
    }

    #[actix_web::test]
    async fn unit_summary_page_should_not_be_shown_when_incorrect_form_data() {
        let tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(_e) => std::process::exit(1),
        };
        let data = web::Data::new(tera);
        let form = web::Form(Info {
            runner_info: RunnerInfo {
                donation: "5".to_string(),
                ..Default::default()
            },
            shipping_info: Default::default(),
        });
        let response = show(form, data).await.unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
