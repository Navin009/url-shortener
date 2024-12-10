use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Url {
    pub long_url: String,
    pub short_code: String,
    pub redirect_count: i32,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct NewUrl {
    pub domain: String,
    pub url: String,
}
