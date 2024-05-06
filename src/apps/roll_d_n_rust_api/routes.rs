use actix_web::web;
use crate::apps::roll_d_n_rust_api::controller;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api").service(
            web::resource("/hello-world").route(
                web::get().to(
                    controller::hello_world
                )
            )
        )
        // Add more routes here
    );
}