use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Url {
    pub long_url: String,
    pub short_code: String,
    pub redirect_count: i32,
    pub expiry_date: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct NewUrl {
    pub expiry: Option<NaiveDateTime>,
    pub url: String,
}
