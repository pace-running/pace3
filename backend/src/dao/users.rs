use crate::models::users::User;
use crate::schema::users::dsl::users;
use crate::schema::users::username;
use diesel::prelude::*;
use diesel::{PgConnection, RunQueryDsl, TextExpressionMethods};

pub fn fetch_user(connection: &mut PgConnection, user_name: String) -> User {
    let database_result = users
        .filter(username.like(user_name))
        .first::<User>(connection);
    return match database_result {
        Ok(user) => user,
        Err(_) => User::default(),
    };
}
