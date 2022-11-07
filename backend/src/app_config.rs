use crate::handlers::{admin, health, runners};
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(web::resource("/health").route(web::get().to(health::health)))
            .service(web::resource("/api/runners").route(web::post().to(runners::create_runner)))
            .service(
                web::resource("/api/runners/{runner_id}").route(web::get().to(runners::get_runner)),
            )
            .service(web::resource("/api/admin/login").route(web::post().to(admin::check_password)))
            .service(web::resource("/api/admin/logout").route(web::post().to(admin::logout)))
            .service(
                web::resource("/api/admin/verification/{runner_id}")
                    .route(web::post().to(admin::verify_payment)),
            )
            .service(web::resource("/api/admin/runners").route(web::get().to(admin::show_runners))),
    );
}
