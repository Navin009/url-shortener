use base64::prelude::*;
use serde_json::json;

use actix_web::{
    body::{BoxBody, MessageBody},
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    web::{self},
    Error, HttpResponse,
};

use crate::config::AppData;

pub async fn basic_auth_middleware(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // pre-processing
    let config = match req.app_data::<web::Data<AppData>>() {
        Some(data) => &data.config,
        None => {
            return Ok(ServiceResponse::new(
                req.request().clone(),
                HttpResponse::InternalServerError().json(json!({
                    "status": "error",
                    "message": "Failed to get config"
                })),
            ));
        }
    };

    let username = config["server"]["authentication"]["basic"]["username"]
        .as_str()
        .unwrap();
    let password = config["server"]["authentication"]["basic"]["password"]
        .as_str()
        .unwrap();

    if let Some(auth_header) = req.headers().get("Authorization") {
        let auth_header = auth_header.to_str().unwrap();

        if auth_header.starts_with("Basic ") {
            let encoded = &auth_header[6..];

            let decoded = String::from_utf8(BASE64_STANDARD.decode(encoded).unwrap()).unwrap();

            let (decoded_username, decoded_password) = decoded.split_once(':').unwrap();

            if decoded_username == username && decoded_password == password {
                return next.call(req).await;
            }
        }

        Ok(ServiceResponse::new(
            req.request().clone(),
            HttpResponse::Unauthorized().finish(),
        ))
    } else {
        Ok(ServiceResponse::new(
            req.request().clone(),
            HttpResponse::Unauthorized().finish(),
        ))
    }
}
