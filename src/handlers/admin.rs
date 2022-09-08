use crate::establish_connection;
use crate::models::users::{LoginData, User};
use actix_web::web::Form;
use actix_web::{web, Error, HttpResponse};
use diesel::prelude::*;

pub async fn login(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let rendered = tmpl
        .render("admin/login.html", &Default::default())
        .unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

pub async fn check_password(login: Form<LoginData>) -> Result<HttpResponse, Error> {
    use crate::schema::users::dsl::*;
    let connection = &mut establish_connection();
    let db_users = users
        .limit(1)
        .load::<User>(connection)
        .expect("Could not load user from database");
    if db_users[0] == login.into_inner() {
        Ok(HttpResponse::Ok()
            .content_type("text/html")
            .body("Matches!"))
    } else {
        Ok(HttpResponse::Forbidden()
            .content_type("text/html")
            .body("wrong password"))
    }
}

pub async fn show_runners(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body("showing runners"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use tera::Tera;

    #[actix_web::test]
    async fn login_form() {
        let tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(_e) => std::process::exit(1),
        };
        let data = actix_web::web::Data::new(tera);
        let resp = login(data).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
