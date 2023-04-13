use crate::core::repository::UserRepository;
use crate::hash_password;
use crate::models::users::User;
use crate::schema::users::dsl::users;
use crate::schema::users::{password_hash as schema_password_hash, username as schema_username}; // talisman-ignore-line
use diesel::r2d2::ConnectionManager;
use diesel::{ExpressionMethods, OptionalExtension, PgConnection, QueryDsl, RunQueryDsl};
use r2d2::Pool;

pub struct PostgresUserRepository {
    connection_pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostgresUserRepository {
    pub fn new(connection_pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { connection_pool }
    }
}

impl UserRepository for PostgresUserRepository {
    fn find_user_by_username(&self, username: String) -> Option<User> {
        let mut connection = self
            .connection_pool
            .get()
            .expect("Unable to get connection.");

        users
            .filter(schema_username.eq(username))
            .get_result::<User>(&mut connection)
            .optional()
            .expect("Failed to find user")
    }

    fn set_password(
        // talisman-ignore-line
        &self,
        username: String,
        new_password: String,
    ) -> anyhow::Result<()> {
        let mut connection = self
            .connection_pool
            .get()
            .expect("Unable to get connection.");

        let new_hash = hash_password(new_password);

        let affected_rows = diesel::update(users)
            .set(schema_password_hash.eq(new_hash))
            .filter(schema_username.eq(&username))
            .execute(&mut connection)
            .expect("Unable to update password");

        if affected_rows == 0 {
            Err(anyhow::Error::msg(
                format!("Username {username} not found.").to_string(),
            ))
        } else {
            Ok(())
        }
    }
}
