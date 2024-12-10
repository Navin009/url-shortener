pub mod ping;
pub mod url;

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(ping::ping)
        .service(url::create_short_url)
        .service(url::redirect)
        .service(url::get_url_details)
        .service(ping::health);
}
