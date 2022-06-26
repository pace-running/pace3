use actix_web::{App,HttpServer};
use actix_web_prom::{PrometheusMetricsBuilder};
use pace::app_config::routes;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .build()
        .unwrap();
    HttpServer::new(move || {
        App::new()
            .wrap(prometheus.clone())
            .configure(routes)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
