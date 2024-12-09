mod config;
mod handlers;
mod models;
mod utils;

use actix_web::{middleware::Logger, App, HttpServer};
use config::init_db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let db = init_db().await.expect("Failed to initialize database");

    HttpServer::new(move || {
        App::new()
            .app_data(db.clone()) // Pass database to app
            .wrap(Logger::default()) // Enable logging middleware
            .configure(handlers::init_routes) // Initialize routes
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
