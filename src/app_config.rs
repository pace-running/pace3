use actix_web::web;

use crate::handlers::{admin, health, hello, join};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(web::resource("/").route(web::get().to(hello::index)))
            .service(web::resource("/join").route(web::get().to(join::form_request)))
            .service(web::resource("/submit").route(web::post().to(join::register)))
            .service(web::resource("/health").route(web::get().to(health::health_endpoint)))
            .service(web::resource("/login").route(web::get().to(admin::login)))
            .service(
                web::resource("/static/{file:([^{}/.]+/)*[^{}/.]+\\.[^{}/]+}")
                    .route(web::get().to(hello::file)),
            ),
    );
}
