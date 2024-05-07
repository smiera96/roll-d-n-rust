mod apps;
mod core;

use std::{env, io};
use actix_web::{App, HttpServer, middleware};
use dotenv::dotenv;
use apps::roll_d_n_rust_api::routes as roll_d_n_rust_api_routes;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let host = find_env("HOST");
    let port = find_env("PORT");
    let url  = format!("{}:{}", host, port);

    match HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(roll_d_n_rust_api_routes::routes)
    })
        .bind(&url) {
            Ok(server) => server.run().await,
            Err(e) => {
                log::error!("Failed to bind server to {}: {:?}", url, e);
                Err(e)
        }
    }
}

fn find_env(key: &str) -> String {
    match env::var(key) {
        Ok(v) => v,
        Err(e) => panic!("${} is not set ({})", key, e)
    }
}