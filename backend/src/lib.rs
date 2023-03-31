use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use dotenvy::dotenv;
use models::rejected_transaction::{NewRejectedTransaction, RejectedTransaction};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use self::models::runner::{NewRunner, Runner};
use self::models::shipping::{NewShipping, Shipping};

pub mod app_config;
pub mod builders;
pub mod constants;
pub mod dao;
pub mod handlers;
pub mod models;
pub mod schema;
pub mod services;

use diesel::r2d2::ConnectionManager;
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    // FIXME: use the connection pool from the application instead
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).unwrap()
}

pub fn has_https() -> bool {
    dotenv().ok();
    let https_enabled =
        env::var("HTTPS_ENABLED").unwrap_or_else(|_error| { "false" }.parse().unwrap());
    https_enabled.eq("true")
}

pub fn session_key() -> String {
    env::var("SESSION_KEY").unwrap_or_else(|_| {
        "DEFAULTSESSIONKEYDONOTUSEINPRODUCTIONORYOUMIGHTDIEAVERYSLOWANDPAINFULLDEATH".to_string()
    })
}

pub fn insert_runner(conn: &mut PgConnection, new_runner: NewRunner) -> Runner {
    diesel::insert_into(schema::runners::table)
        .values(&new_runner)
        .get_result(conn)
        .expect("Error saving runner")
}

pub fn insert_rejected_transaction(
    conn: &mut PgConnection,
    new_transaction: NewRejectedTransaction,
) -> RejectedTransaction {
    diesel::insert_into(schema::rejected_transactions::table)
        .values(&new_transaction)
        .get_result(conn)
        .expect("Error saving transaction")
}

pub fn retrieve_runner_by_id(conn: &mut PgConnection, id: i32) -> Runner {
    use crate::schema::runners::dsl::runners;

    runners
        .find(id)
        .get_result::<Runner>(conn)
        .unwrap_or_else(|_| panic!("Could not retrieve runner with id {}", id))
}

pub fn retrieve_donation_by_reason_for_payment(
    conn: &mut PgConnection,
    rfp: &str,
) -> Result<Runner, Error> {
    use crate::schema::runners::dsl::*;
    runners
        .filter(reason_for_payment.eq(rfp))
        .get_result::<Runner>(conn)
}

pub fn retrieve_shipping_by_runner_id(
    conn: &mut PgConnection,
    queried_id: i32,
) -> Result<Shipping, Error> {
    use crate::schema::shippings::dsl::*;

    shippings
        .filter(runner_id.eq(queried_id))
        .get_result::<Shipping>(conn)
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
        .expect("Error getting start number")
        .start_number
}

pub fn is_eu_country(country: &str) -> bool {
    let eu_countries = [
        "Belgien",
        "Bulgarien",
        "Dänemark",
        "Estland",
        "Finnland",
        "Frankreich",
        "Griechenland",
        "Irland",
        "Italien",
        "Kroatien",
        "Lettland",
        "Litauen",
        "Luxemburg",
        "Malta",
        "Niederlande",
        "Österreich",
        "Polen",
        "Portugal",
        "Rumänien",
        "Schweden",
        "Slowakei",
        "Slowenien",
        "Spanien",
        "Tschechische Republik",
        "Ungarn",
        "Zypern",
    ];
    eu_countries.contains(&country)
}

pub fn hash_password(password: String) -> String {
    let config = argon2::Config::default();
    let salt: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();

    argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config).unwrap()
}