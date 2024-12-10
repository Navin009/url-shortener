mod config;
mod handlers;
mod middleware;
mod models;
mod utils;

use actix_web::{
    http::StatusCode,
    middleware::{ErrorHandlers, Logger},
    web::Data,
    App, HttpServer,
};
use config::init_db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let db = init_db().await.expect("Failed to initialize database");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(db.clone())) // Pass database to app
            .wrap(Logger::default()) // Enable logging middleware
            .wrap(ErrorHandlers::new().handler(
                StatusCode::BAD_REQUEST,
                middleware::validation_handler::add_error_body,
            ))
            .configure(handlers::init_routes) // Initialize routes
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
