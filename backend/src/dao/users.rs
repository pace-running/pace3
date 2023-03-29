use crate::models::users::User;
use crate::schema::users::dsl::users;
use crate::schema::users::username;
use crate::DbPool;
use diesel::prelude::*;
use diesel::{RunQueryDsl, TextExpressionMethods};
use mockall::*;
use mockall::predicate::*;

#[derive(Clone)]
pub struct Dao {
    pool: DbPool,
}

#[automock]
pub trait UserDAOTrait {
//    fn new(pool: DbPool) -> Dao;
    fn fetch_user(&self, user_name: String) -> User;
}

impl Dao {
    pub fn new(pool: DbPool) -> Dao {
        return Dao { pool }
    }
}

impl UserDAOTrait for Dao {

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
