use actix_web::web;

use crate::handlers::{hello, health };

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(
                web::resource("/")
                    .route(web::get().to(hello::hello_world))
            )
            .service(
                web::resource("/hey")
                    .route(web::get().to(hello::hey))
            )
            .service(
                web::resource("/echo")
                    .route(web::post().to(hello::echo))
            )
            .service(
                web::resource("/health")
                    .route(web::get().to(health::health_endpoint))
            )
    );
}