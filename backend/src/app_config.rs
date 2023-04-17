use crate::handlers::{admin, health, runners, theme};
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
                web::resource("/api/admin/change_password")
                    .route(web::put().to(admin::change_password)),
            )
            .service(
                web::resource("/api/admin/payment/{runner_id}")
                    .route(web::post().to(admin::modify_payment_status)),
            )
            .service(web::resource("/api/admin/runners").route(web::get().to(admin::show_runners)))
            .service(
                web::resource("/api/admin/full_runner/{runner_id}")
                    .route(web::get().to(admin::get_full_runner))
                    .route(web::put().to(admin::edit_runner)),
            )
            .service(
                web::resource("/api/admin/finance")
                    .route(web::post().to(admin::parse_payment_csv))
                    .route(web::get().to(admin::get_rejected_transactions)),
            )
            .service(
                web::resource("/api/theme")
                    .route(web::get().to(theme::get_theme))
                    .route(web::put().to(theme::update_theme)),
            ),
    );
}
