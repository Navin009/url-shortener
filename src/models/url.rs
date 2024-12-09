use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Url {
    pub id: ObjectId,
    pub long_url: String,
    pub short_code: String,
    pub redirect_count: i32,
}

#[derive(Deserialize)]
pub struct NewUrl {
    pub long_url: String,
}
