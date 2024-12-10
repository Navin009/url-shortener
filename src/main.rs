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
use config::{init_app_config, init_db, AppData};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = init_app_config()
        .await
        .expect("Failed to initialize config");

    let log_level = config["logging"]["level"].as_str().unwrap();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or(log_level));

    let db = init_db(&config)
        .await
        .expect("Failed to initialize database");

    let app_data = AppData {
        db: db.clone(),
        config: config.clone(),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(app_data.clone())) // Pass database to app
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
