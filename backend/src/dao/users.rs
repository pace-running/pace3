use crate::models::users::User;
use crate::schema::users::dsl::users;
use crate::schema::users::username;
use crate::DbPool;
use diesel::prelude::*;
use diesel::{RunQueryDsl, TextExpressionMethods};

#[derive(Clone)]
pub struct Dao {
    pool: DbPool,
}

pub trait UserDAOTrait {
    fn new(pool: DbPool) -> Dao;
    fn fetch_user(&self, user_name: String) -> User;
}

impl UserDAOTrait for Dao {
    fn new(pool: DbPool) -> Dao {
        Dao { pool: pool }
    }

    fn fetch_user(&self, user_name: String) -> User {
        let connection = &mut self
            .pool
            .get()
            .expect("couldn't get db connection from pool");
        let database_result = users
            .filter(username.like(user_name))
            .first::<User>(connection);
        return match database_result {
            Ok(user) => user,
            Err(_) => User::default(),
        };
    }
}
