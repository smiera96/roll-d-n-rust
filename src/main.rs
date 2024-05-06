use actix_web::{App, HttpServer, middleware};
use apps::roll_d_n_rust_api::routes as roll_d_n_rust_api_routes;

mod apps;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default()) // Middleware for logging
            .configure(roll_d_n_rust_api_routes::routes) // Configuring routes
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}