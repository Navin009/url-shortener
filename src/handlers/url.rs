use crate::models::url::{NewUrl, Url};
use crate::utils::shortener::generate_short_code;
use actix_web::{get, post, web, HttpResponse};
use mongodb::bson::doc;
use mongodb::Collection;
use serde_json::json;

#[post("/create")]
pub async fn create_short_url(
    db: web::Data<mongodb::Database>,
    payload: web::Json<NewUrl>,
) -> HttpResponse {
    log::info!("Received payload: {:#?}", payload.url);

    let collection: Collection<Url> = db.collection("urls");
    let short_code = generate_short_code();
    let new_url = Url {
        long_url: payload.url.clone(),
        short_code: short_code.clone(),
        redirect_count: 0,
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
pub async fn redirect(
    db: web::Data<mongodb::Database>,
    short_code: web::Path<String>,
) -> HttpResponse {
    let collection: Collection<Url> = db.collection("urls");
    let short_code = short_code.into_inner();

    match collection
        .find_one_and_update(
            doc! { "short_code": &short_code },
            doc! { "$inc": { "redirect_count": 1 } },
        )
        .await
    {
        Ok(Some(url)) => HttpResponse::PermanentRedirect()
            .append_header(("Location", url.long_url))
            .finish(),
        Ok(None) => HttpResponse::NotFound().json("Short URL not found"),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

#[get("/details/{short_code}")]
pub async fn get_url_details(
    db: web::Data<mongodb::Database>,
    short_code: web::Path<String>,
) -> HttpResponse {
    let collection: Collection<Url> = db.collection("urls");
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
