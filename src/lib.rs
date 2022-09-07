use self::models::runner::{NewRunner, Runner};
use self::models::shipping::{NewShipping, Shipping};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod app_config;
pub mod builders;
pub mod converters;
pub mod handlers;
pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).unwrap()
}

pub fn insert_runner(conn: &mut PgConnection, new_runner: NewRunner) -> Runner {
    use crate::schema::runners;

    diesel::insert_into(runners::table)
        .values(&new_runner)
        .get_result(conn)
        .expect("Error saving runner")
}

pub fn insert_shipping(conn: &mut PgConnection, new_shipping: NewShipping) -> Shipping {
    use crate::schema::shippings;

    diesel::insert_into(shippings::table)
        .values(&new_shipping)
        .get_result(conn)
        .expect("Error saving shipping")
}

pub fn get_next_start_number(conn: &mut PgConnection) -> i64 {
    use self::models::start_number::StartNumber;
    use diesel::sql_query;

    sql_query("SELECT nextval('runner_start_number_seq') AS start_number")
        .get_result::<StartNumber>(conn)
        .expect("Error getting startnumber")
        .start_number
}

// For testing only
pub fn restart_start_number(conn: &mut PgConnection) {
    use diesel::sql_query;

    sql_query("ALTER SEQUENCE runner_start_number_seq RESTART")
        .execute(conn)
        .expect("Error resetting start_number sequence");
}
