use crate::config::AppData;
use crate::models::url::{NewUrl, Url};
use crate::utils::shortener::generate_short_code;
use actix_web::{get, web, HttpResponse};
use mongodb::bson::doc;
use mongodb::Collection;
use serde_json::json;

pub async fn create_short_url(
    state: web::Data<AppData>,
    payload: web::Json<NewUrl>,
) -> HttpResponse {
    let collection: Collection<Url> = state.db.collection("urls");
    let short_code = generate_short_code();
    let new_url = Url {
        long_url: payload.url.clone(),
        short_code: short_code.clone(),
        redirect_count: 0,
        created_at: chrono::Utc::now().naive_utc(),
        expiry_date: payload.expiry,
    };

    let short_code = match collection.insert_one(new_url).await {
        Ok(_) => {
            println!("Short code created successfully.");
            short_code
        }

        Err(err) => {
            return HttpResponse::InternalServerError().json(json!({
                "status": "error",
                "message": err.to_string()
            }));
        }
    };

    HttpResponse::Ok().json(json!({
        "status": "success",
        "short_code": short_code
    }))
}

#[get("/{short_code}")]
pub async fn referral_redirect(
    state: web::Data<AppData>,
    short_code: web::Path<String>,
) -> HttpResponse {
    let collection: Collection<Url> = state.db.collection("urls");
    let short_code = short_code.into_inner();

    match collection
        .find_one_and_update(
            doc! { "short_code": &short_code },
            doc! { "$inc": { "redirect_count": 1 } },
        )
        .await
    {
        Ok(Some(url)) => match url.expiry_date {
            Some(expiry_date) => {
                if expiry_date < chrono::Utc::now().naive_utc() {
                    HttpResponse::Gone().finish()
                } else {
                    HttpResponse::PermanentRedirect()
                        .append_header(("Location", url.long_url))
                        .finish()
                }
            }
            None => HttpResponse::PermanentRedirect()
                .append_header(("Location", url.long_url))
                .finish(),
        },
        Ok(None) => HttpResponse::NotFound().json("Short URL not found"),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

#[get("/details/{short_code}")]
pub async fn get_url_details(
    state: web::Data<AppData>,
    short_code: web::Path<String>,
) -> HttpResponse {
    let collection: Collection<Url> = state.db.collection("urls");
    let short_code = short_code.into_inner();

    match collection
        .find_one(doc! { "short_code": &short_code })
        .await
    {
        Ok(Some(url)) => HttpResponse::Ok().json(url),
        Ok(None) => HttpResponse::NotFound().json("Short URL not found"),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}
