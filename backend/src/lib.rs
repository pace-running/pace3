use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use dotenvy::dotenv;
use models::rejected_transaction::{NewRejectedTransaction, RejectedTransaction};
use r2d2::PooledConnection;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use repository::PostgresThemeRepository;

use self::models::runner::Runner;
use self::models::shipping::{NewShipping, Shipping};

pub mod app_config;
pub mod builders;
pub mod constants;
pub mod core;
pub mod handlers;
pub mod models;
pub mod repository;
pub mod schema;
pub mod services;

use crate::app_config::routes;
use crate::core::service::{
    DefaultPaymentService, DefaultRunnerService, DefaultThemeService, DefaultUserService,
    EmailConfiguration, EmailService, LettreTeraEmailService, NonfunctionalEmailService,
    PaymentService, RunnerService, ThemeService, UserService,
};
use crate::repository::{
    PostgresPaymentRepository, PostgresRunnerRepository, PostgresUserRepository,
};
use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::dev::Server;
use actix_web::{http, web, App, HttpServer};
use actix_web_prom::PrometheusMetricsBuilder;
use diesel::r2d2::ConnectionManager;
use lazy_static::lazy_static;
use std::net::TcpListener;
use std::sync::Arc;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DatabaseConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn get_connection_pool() -> Result<DbPool, r2d2::Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection_manager = ConnectionManager::<PgConnection>::new(database_url);
    // TODO: store in lazy loaded singleton
    r2d2::Pool::builder().build(connection_manager)
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

pub fn insert_rejected_transaction(
    conn: &mut PgConnection,
    new_transaction: NewRejectedTransaction,
) -> RejectedTransaction {
    diesel::insert_into(schema::rejected_transactions::table)
        .values(&new_transaction)
        .get_result(conn)
        .expect("Error saving transaction")
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
        .into()
}

pub const EU_COUNTRIES: [&str; 26] = [
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

pub fn is_eu_country(country: &str) -> bool {
    EU_COUNTRIES.contains(&country)
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

lazy_static! {
    static ref TEMPLATES: tera::Tera = {
        match tera::Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(_e) => std::process::exit(1),
        }
    };
}

pub fn run(listener: TcpListener, db_pool: DbPool) -> Result<Server, std::io::Error> {
    let secret_key = Key::from(session_key().as_ref());
    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .build()
        .unwrap();
    let server = HttpServer::new(move || {
        let email_configuration = EmailConfiguration::from_env();
        let email_service: Arc<dyn EmailService> = if let Ok(configuration) = email_configuration {
            Arc::new({
                LettreTeraEmailService::new(configuration, &TEMPLATES, None)
                    .expect("Unable to instantiate EmailService")
            })
        } else {
            Arc::new(NonfunctionalEmailService {})
        };

        let runner_repository = PostgresRunnerRepository::new(db_pool.clone());
        let runner_service: Arc<dyn RunnerService> =
            Arc::new(DefaultRunnerService::new(runner_repository));

        let user_repository = PostgresUserRepository::new(db_pool.clone());
        let user_service: Arc<dyn UserService> = Arc::new(DefaultUserService::new(user_repository));

        let theme_repository = PostgresThemeRepository::new(db_pool.clone());
        let theme_service: Arc<dyn ThemeService> =
            Arc::new(DefaultThemeService::new(theme_repository));

        let payment_repository = PostgresPaymentRepository::new(db_pool.clone());
        let payment_service: Arc<dyn PaymentService> =
            Arc::new(DefaultPaymentService::new(payment_repository));

        let session_middleware =
            SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                .cookie_secure(has_https())
                .build();
        let cors = Cors::default()
            .allowed_origin_fn(|origin, _req_head| {
                origin.as_bytes().ends_with(b"localhost:3000")
                    || origin.as_bytes().ends_with(b"localhost:8089")
                    || origin.as_bytes().ends_with(b"stadtpark-marathon.de")
                    || origin.as_bytes().ends_with(b"lauf-gegen-rechts.de")
            })
            .allowed_methods(vec!["GET", "POST", "PUT"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
                http::header::CONTENT_TYPE,
            ])
            .max_age(3600);

        App::new()
            .wrap(IdentityMiddleware::default())
            .wrap(session_middleware)
            .wrap(prometheus.clone())
            .wrap(cors)
            .configure(routes)
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::from(payment_service))
            .app_data(web::Data::from(runner_service))
            .app_data(web::Data::from(user_service))
            .app_data(web::Data::from(theme_service))
            .app_data(web::Data::from(email_service))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
