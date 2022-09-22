use crate::establish_connection;
use crate::models::users::{LoginData, LoginResponse, User};
use actix_web::web::Json;
use actix_web::{Error, HttpResponse};
use diesel::prelude::*;

pub async fn check_password(login: Json<LoginData>) -> Result<HttpResponse, Error> {
    use crate::schema::users::dsl::*;
    let connection = &mut establish_connection();
    let db_users = users
        .limit(1)
        .load::<User>(connection)
        .expect("Could not load user from database");
    if db_users[0] == login.into_inner() {
        let response = LoginResponse::from(&db_users[0]);
        let json = serde_json::to_string(&response)?;
        Ok(HttpResponse::Ok().content_type("text/json").body(json))
    } else {
        Ok(HttpResponse::Forbidden()
            .content_type("text/json")
            .body("{'result': fail}"))
    }
}

pub async fn show_runners() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .content_type("text/json")
        .body("{'showing runners'}"))
}
