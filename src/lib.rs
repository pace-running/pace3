use self::models::runner::{NewRunner, Runner};
use self::models::shipping::{NewShipping, Shipping};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::sql_types::Integer;
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

pub fn get_next_start_number(conn: &mut PgConnection) -> i32 {
    use diesel::sql_query;
    use self::models::start_number::StartNumber;

    sql_query("SELECT nextval('runner_start_number_seq') AS start_number")
    .get_result::<StartNumber>(conn)
    .expect("Error getting startnumber")
    .start_number
}

pub fn set_last_start_number(conn: &mut PgConnection, last: &i32) {
    use diesel::sql_query;

    sql_query("SELECT setval('runner_start_number_seq', ?)")
    .bind::<Integer, &i32>(last)
    .execute(conn)
    .expect("Error setting the latest start_number");
}
