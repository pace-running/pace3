use crate::models::users::User;
use crate::schema::users::dsl::users;
use crate::schema::users::{password_hash, username};
use crate::DbPool;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

#[derive(Clone)]
pub struct Dao {
    pool: DbPool,
}

fn hash_password(password: String) -> String {
    let config = argon2::Config::default();
    let salt: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();

    argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config).unwrap()
}

pub trait UserDAOTrait {
    fn new(pool: DbPool) -> Dao;
    fn fetch_user(&self, user_name: String) -> User;
    fn set_password(&self, user_name: String, new_password: String);
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
