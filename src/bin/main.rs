use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{http, App, HttpServer};
use actix_web_prom::PrometheusMetricsBuilder;
use pace::app_config::routes;
use tera::Tera;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .build()
        .unwrap();
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin_fn(|origin, _req_head| {
                origin.as_bytes().ends_with(b"localhost:3000")
                    || origin.as_bytes().ends_with(b"lauf-gegen-rechts.de")
            })
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
                http::header::CONTENT_TYPE,
            ])
            .max_age(3600);

        let mut tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html", ".sql"]);
        App::new()
            .app_data(Data::new(tera))
            .wrap(prometheus.clone())
            .wrap(cors)
            .configure(routes)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
