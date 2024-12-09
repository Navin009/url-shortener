use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Serialize, Deserialize)]
pub struct Url {
    pub long_url: String,
    pub short_code: String,
    pub redirect_count: i32,
}

#[derive(Deserialize, Validate)]
pub struct NewUrl {
    #[validate(custom(function = "domain_check"))]
    pub domain: String,
    pub url: String,
}

fn domain_check(domain: &str) -> Result<(), ValidationError> {
    if domain != "r" && domain != "p" && domain != "d" {
        return Err(ValidationError::new("Invalid domain"));
    }

    Ok(())
}