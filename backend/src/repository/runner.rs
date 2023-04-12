use crate::core::repository::{RunnerId, RunnerRepository};
use crate::models::runner::{NewRunner, Runner};
use crate::schema;
use crate::schema::runners::dsl::runners;
use diesel::r2d2::ConnectionManager;
use diesel::{OptionalExtension, PgConnection, QueryDsl, RunQueryDsl};
use r2d2::Pool;

pub struct PostgresRunnerRepository {
    connection_pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostgresRunnerRepository {
    pub fn new(connection_pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { connection_pool }
    }
}

impl RunnerRepository for PostgresRunnerRepository {
    fn insert_runner(&self, new_runner: NewRunner) -> Runner {
        let mut connection = self
            .connection_pool
            .get()
            .expect("Unable to get connection.");

        diesel::insert_into(schema::runners::table)
            .values(&new_runner)
            .get_result(&mut connection)
            .expect("Error saving runner")
    }

    fn find_runner_by_id(&self, id: RunnerId) -> Option<Runner> {
        let mut connection = self
            .connection_pool
            .get()
            .expect("Unable to get connection.");

        runners
            .find(id)
            .get_result::<Runner>(&mut connection)
            .optional()
            .expect("Failed to find runner")
    }
}
