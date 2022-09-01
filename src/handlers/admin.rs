use actix_web::{Error,HttpResponse,web};
use tera::Context;
use crate::models::event;

pub async fn login(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse,Error> {
    let mut ctx = Context::new();
    ctx.insert("event", &event::current_event());
    let rendered = tmpl.render("admin/login.html", &ctx).unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))

}

#[cfg(test)]
mod tests {
    use tera::Tera;
    use actix_web::http::StatusCode;
    use super::*;
    #[actix_web::test]
    async fn form_page() {
        let tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(_e) => { std::process::exit(1) }
        };
        let data = actix_web::web::Data::new(tera);
        let resp = login(data).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}