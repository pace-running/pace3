use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use dotenvy::dotenv;
use models::rejected_transaction::{NewRejectedTransaction, RejectedTransaction};
use r2d2::PooledConnection;
use repository::PostgresThemeRepository;

use self::models::runner::Runner;

pub mod app_config;
pub mod builders;
pub mod constants;
pub mod core;
pub mod handlers;
pub mod models;
pub mod repository;
pub mod schema;
pub mod validation;

use crate::app_config::routes;
use crate::core::repository::ThemeRepository;
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

pub fn get_connection_pool(database_url: Option<String>) -> Result<DbPool, r2d2::Error> {
    dotenv().ok();

    let database_url = database_url.unwrap_or_else(|| env::var("DATABASE_URL").expect("Database connection string neither provided via parameter `database_url` nor via env variable `DATABASE_URL`"));
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

lazy_static! {
    static ref TEMPLATES: tera::Tera = {
        match tera::Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(_e) => std::process::exit(1),
        }
    };
}

pub fn run(
    listener: TcpListener,
    db_pool: DbPool,
    email_configuration: Option<EmailConfiguration>,
) -> Result<Server, std::io::Error> {
    let secret_key = Key::from(session_key().as_ref());
    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .build()
        .unwrap();
    let server = HttpServer::new(move || {
        let theme_repository: Arc<dyn ThemeRepository> =
            Arc::new(PostgresThemeRepository::new(db_pool.clone()));

        let email_service: Arc<dyn EmailService> = if let Some(configuration) = email_configuration
            .clone()
            .or_else(|| EmailConfiguration::from_env().ok())
        {
            Arc::new({
                LettreTeraEmailService::new(
                    configuration,
                    &TEMPLATES,
                    None,
                    theme_repository.clone(),
                )
                .expect("Unable to instantiate EmailService")
            })
        } else {
            Arc::new(NonfunctionalEmailService {})
        };

        let runner_repository = PostgresRunnerRepository::new(db_pool.clone());
        let runner_service: Arc<dyn RunnerService> = Arc::new(DefaultRunnerService::new(
            runner_repository,
            theme_repository.clone(),
            email_service.clone(),
        ));

        let user_repository = PostgresUserRepository::new(db_pool.clone());
        let user_service: Arc<dyn UserService> = Arc::new(DefaultUserService::new(user_repository));

        let theme_service: Arc<dyn ThemeService> =
            Arc::new(DefaultThemeService::new(theme_repository.clone()));

        let payment_repository = PostgresPaymentRepository::new(db_pool.clone());
        let payment_service: Arc<dyn PaymentService> = Arc::new(DefaultPaymentService::new(
            payment_repository,
            email_service.clone(),
        ));

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
