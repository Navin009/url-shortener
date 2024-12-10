use actix_web::{get, HttpResponse};
use serde_json::json;

#[get("/ping")]
pub async fn ping() -> HttpResponse {
    HttpResponse::Ok().json("pong")
}

#[get("/health")]
pub async fn health() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "status": "UP"
    }))
}
