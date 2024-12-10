pub mod ping;
pub mod url;

use actix_web::{middleware, web};

use crate::middleware::basic_auth;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(ping::ping)
        .service(
            web::resource("/create")
                .wrap(middleware::from_fn(basic_auth::basic_auth_middleware))
                .to(url::create_short_url),
        )
        .service(url::referral_redirect)
        .service(url::get_url_details)
        .service(ping::health);
}
