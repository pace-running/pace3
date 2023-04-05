use crate::models::users::User;
use crate::schema::users::dsl::users;
use crate::schema::users::{password_hash, username};
use crate::{hash_password, DbPool};
use diesel::prelude::*;
use diesel::RunQueryDsl;
use mockall::predicate::*;
use mockall::*;

#[derive(Clone)]
pub struct Dao {
    pool: DbPool,
}

pub trait UserDAOTrait {
    fn fetch_user(&self, user_name: String) -> User;
    fn set_password(&self, user_name: String, new_password: String);
}

impl Dao {
    pub fn new(pool: DbPool) -> Dao {
        return Dao { pool };
    }

    pub fn pool(&self) -> DbPool {
        self.pool.clone()
    }
}

#[automock]
impl UserDAOTrait for Dao {
    fn fetch_user(&self, user_name: String) -> User {
        let connection = &mut self
            .pool
            .get()
            .expect("couldn't get db connection from pool");
        let database_result = users
            .filter(username.eq(user_name))
            .first::<User>(connection);
        return match database_result {
            Ok(user) => user,
            Err(_) => User::default(),
        };
    }

    fn set_password(&self, user_name: String, new_password: String) {
        let new_hash = hash_password(new_password);
        let connection = &mut self
            .pool
            .get()
            .expect("couldn't get db connection from pool");
        diesel::update(users)
            .set(password_hash.eq(new_hash))
            .filter(username.eq(user_name))
            .execute(connection)
            .expect("could not update password for user");
    }
}
