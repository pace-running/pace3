use crate::establish_connection;
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
    if user == login_data.into_inner() {
        let response = LoginResponse::from(&user);
        let json = serde_json::to_string(&response)?;
        Identity::login(&request.extensions(), response.username.to_string()).unwrap();
        Ok(HttpResponse::Ok().content_type("text/json").body(json))
    } else {
        Ok(HttpResponse::Forbidden()
            .content_type("text/json")
            .body("\"result\": \"fail\""))
    }
}

pub async fn show_runners() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .content_type("text/json")
        .body("{'showing runners'}"))
}
