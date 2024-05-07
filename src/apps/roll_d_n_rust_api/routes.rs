use actix_web::web;
use crate::apps::roll_d_n_rust_api::controllers::{dice_controller, health_check_controller};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::resource("/health-check").route(web::get().to(health_check_controller::health_check)))
            .service(
                web::scope("/dice")
                    .service(web::resource("/roll").route(web::get().to(dice_controller::roll_dice)))
            )
    );
}