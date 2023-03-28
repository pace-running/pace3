use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, http, web, App, HttpServer};
use actix_web_prom::PrometheusMetricsBuilder;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use dotenvy::dotenv;
use pace::app_config::routes;
use pace::dao::users::{Dao, UserDAOTrait};
use pace::{has_https, session_key};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection_manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(connection_manager)
        .expect("Could not build connection pool");
    let dao = Dao::new(pool);

    let secret_key = Key::from(session_key().as_ref());
    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .build()
        .unwrap();
    HttpServer::new(move || {
        let session_middleware =
            SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                .cookie_secure(has_https())
                .build();
        let cors = Cors::default()
            .allowed_origin_fn(|origin, _req_head| {
                origin.as_bytes().ends_with(b"localhost:3000")
                    || origin.as_bytes().ends_with(b"localhost:8089")
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
            .app_data(web::Data::new(dao.clone()))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
