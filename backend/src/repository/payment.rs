use crate::core::repository::PaymentRepository;
use crate::schema;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::{PgConnection, RunQueryDsl};
use r2d2::Pool;

pub struct PostgresPaymentRepository {
    connection_pool: Pool<ConnectionManager<PgConnection>>,
}

impl PostgresPaymentRepository {
    pub fn new(connection_pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { connection_pool }
    }
}

impl PaymentRepository for PostgresPaymentRepository {
    fn delete_rejected_transactions_by_ids(&self, rejected_transactions_ids: Vec<i32>) -> usize {
        let mut connection = self
            .connection_pool
            .get()
            .expect("Unable to get connection");

        diesel::delete(schema::rejected_transactions::table)
            .filter(schema::rejected_transactions::id.eq_any(rejected_transactions_ids))
            .execute(&mut connection)
            .expect("Unable to delete rejected transactions")
    }
}
