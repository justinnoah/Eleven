use iron::prelude::*;
use iron::status;
use serde_json as json;

include!(concat!(env!("OUT_DIR"), "/data_types.rs"));

// Routes needed for r0.2.0
//
// error response
fn api_error(errorcode: ApiError, error: &str, code: status::Status) -> IronResult<Response> {
    let e_resp = ApiErrorResponse {
        errorcode: errorcode,
        error: error.to_string(),
    };
    debug!("Error: {:?}", json::to_string(&e_resp).unwrap());

    Ok(Response::with((code, json::to_string(&e_resp).unwrap())))
}

// Common use for the start of this project - a nice place holder
fn not_yet_implemented() -> IronResult<Response> {
    api_error(ApiError::M_NOT_IMPLEMENTED, "Not Yet Implemented", status::BadRequest)
}

// Forbidden made easy
fn forbidden_call(reason: &str) -> IronResult<Response> {
    api_error(ApiError::M_FORBIDDEN, reason, status::Forbidden)
}
