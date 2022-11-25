use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, http, App, HttpServer};
use actix_web_prom::PrometheusMetricsBuilder;
use pace::app_config::routes;
use pace::{has_https, session_key};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
                    || origin.as_bytes().ends_with(b"lauf-gegen-rechts.de")
            })
            .allowed_methods(vec!["GET", "POST","PUT"])
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
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
