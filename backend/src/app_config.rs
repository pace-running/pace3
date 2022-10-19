use crate::handlers::{admin, health, hello, imprint, join, privacy_notice, runner, summary};
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(web::resource("/").route(web::get().to(hello::index)))
            .service(web::resource("/join").route(web::get().to(join::form_request)))
            .service(web::resource("/api/submit").route(web::post().to(join::register)))
            .service(
                web::resource("/api/runner/{runner_id}").route(web::get().to(runner::get_runner)),
            )
            .service(web::resource("/summary").route(web::post().to(summary::show)))
            .service(web::resource("/health").route(web::get().to(health::health_endpoint)))
            .service(web::resource("/api/admin/login").route(web::post().to(admin::check_password)))
            .service(web::resource("/api/admin/logout").route(web::post().to(admin::logout)))
            .service(web::resource("/api/admin/runners").route(web::get().to(admin::show_runners)))
            .service(web::resource("/imprint").route(web::get().to(imprint::show)))
            .service(web::resource("/privacy_notice").route(web::get().to(privacy_notice::show)))
            .service(
                web::resource("/static/{file:([^{}/.]+/)*[^{}/.]+\\.[^{}/]+}")
                    .route(web::get().to(hello::file)),
            ),
    );
}
