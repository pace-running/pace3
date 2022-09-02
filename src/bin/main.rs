use actix_web::{App,HttpServer};
use actix_web::web::Data;
use actix_web_prom::{PrometheusMetricsBuilder};
use pace::app_config::routes;
use tera::{Tera};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .build()
        .unwrap();
    HttpServer::new(move || {
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
            .configure(routes)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
