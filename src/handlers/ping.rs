use actix_web::{get, HttpResponse};

#[get("/ping")]
pub async fn ping() -> HttpResponse {
    HttpResponse::Ok().json("pong")
}
