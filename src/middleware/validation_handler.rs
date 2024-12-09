use actix_web::{dev::ServiceResponse, http::header, middleware::ErrorHandlerResponse, Result};
use serde_json::json;

pub fn add_error_body<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let error_msg: String = match res.response().error() {
        Some(e) => e.to_string(),
        None => String::from("Unknown Error"),
    };

    // split service response into request and response components
    let (req, mut res) = res.into_parts();

    res.headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );

    // set body of response to modified body
    let res = res.set_body(
        json!({
            "status": "ValidationError",
            "message": error_msg.to_string()
        })
        .to_string(),
    );

    // modified bodies need to be boxed and placed in the "right" slot
    let res = ServiceResponse::new(req, res)
        .map_into_boxed_body()
        .map_into_right_body();

    Ok(ErrorHandlerResponse::Response(res))
}
