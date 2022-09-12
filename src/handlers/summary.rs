use super::join::has_bad_data;
use crate::models::{event, info::Info};
use actix_web::{web, Error, HttpResponse, Result};
use tera::Context;

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
