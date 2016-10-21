use std::collections::HashMap;

use iron::prelude::*;
use iron::Handler;
use iron::status;
use iron::method::Method;
use rustc_serialize::json;

use endpoint::client_types::{ApiError, ApiErrorResponse};

// Routes needed for r0.2.0
//
// error response
fn api_error(errorcode: ApiError, error: &str, code: status::Status) -> IronResult<Response> {
    let e_resp = ApiErrorResponse {
        errorcode: errorcode,
        error: error.to_string(),
    };
    debug!("Error: {:?}", json::encode(&e_resp));

    Ok(Response::with((code, json::encode(&e_resp).unwrap())))
}

// Login API
// URL: _matrix/client/r0/lugin
pub fn login(req: &mut Request) -> IronResult<Response> {
    debug!("User Logging In!");
    match req.method {
        Method::Post => {
            api_error(ApiError::M_NOT_IMPLEMENTED, "Not Yet Implemented", status::BadRequest)
        },
        _ => {
            api_error(ApiError::M_FORBIDDEN, "One must POST to login", status::Forbidden)
        },
    }
}

// Setup the client routes
pub fn load_client_routes() -> HashMap<String, Box<Handler>> {
    let mut routes: HashMap<String, Box<Handler>> = HashMap::new();
    routes.insert("r0/login".to_string(), Box::new(login));

    routes
}
