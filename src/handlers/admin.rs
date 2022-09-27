use crate::establish_connection;
use crate::models::runner::Runner;
use crate::models::users::{LoginData, LoginResponse, User};
use actix_identity::Identity;
use actix_web::web::Json;
use actix_web::{Error, HttpMessage, HttpRequest, HttpResponse};
use diesel::prelude::*;

pub async fn check_password(
    request: HttpRequest,
    login_data: Json<LoginData>,
) -> Result<HttpResponse, Error> {
    use crate::schema::users::dsl::*;
    let connection = &mut establish_connection();
    let database_result = users
        .filter(username.like(&login_data.username))
        .first::<User>(connection);
    let user = match database_result {
        Ok(user) => user,
        Err(_) => User::default(),
    };
    if user.eq(&login_data.into_inner()) {
        let response = LoginResponse::from(&user);
        let json = serde_json::to_string(&response)?;
        Identity::login(&request.extensions(), response.username).unwrap();
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(json))
    } else {
        Ok(forbidden())
    }
}

pub async fn show_runners(_: Identity) -> Result<HttpResponse, Error> {
    use crate::schema::runners::dsl::*;
    let connection = &mut establish_connection();
    let database_result = runners.load::<Runner>(connection);
    Ok(HttpResponse::Ok()
        .content_type("text/json")
        .body(serde_json::to_string(&database_result.unwrap()).unwrap()))
}

pub async fn logout(user: Identity) -> Result<HttpResponse, Error> {
    user.logout();
    Ok(HttpResponse::NoContent().finish())
}

fn forbidden() -> HttpResponse {
    HttpResponse::Forbidden()
        .content_type("application/json")
        .body("\"result\": \"fail\"")
}
