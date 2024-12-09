use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Url {
    #[serde(rename = "_id", skip_serializing)]
    pub id: ObjectId,
    pub long_url: String,
    pub short_code: String,
    pub redirect_count: i32,
}

#[derive(Deserialize)]
pub struct NewUrl {
    pub url: String,
}
