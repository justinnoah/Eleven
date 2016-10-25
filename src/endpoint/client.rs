use std::collections::HashMap;

use iron::prelude::*;
use iron::Handler;
use iron::status;
use iron::method::Method;
use serde_json as json;
use slog_envlogger;

// use endpoint::client_types::{ApiError, ApiErrorResponse};
#[cfg(feature = "serde_derive")]
include!("client_types.in.rs");

#[cfg(feature = "serde_codegen")]
include!(concat!(env!("OUT_DIR"), "/client_types.rs"));

// Routes needed for r0.2.0
//
// error response
fn api_error(errorcode: ApiError, error: &str, code: status::Status) -> IronResult<Response> {
    let e_resp = ApiErrorResponse {
        errorcode: errorcode,
        error: error.to_string(),
    };
    debug!("Error: {:?}", json::to_string(&e_resp).unwrap().to_string());

    Ok(Response::with((code, json::to_string(&e_resp).unwrap())))
}

// Common use for the start of this project - a nice place holder
fn not_yet_implemented() -> IronResult<Response> {
    api_error(ApiError::M_NOT_IMPLEMENTED, "Not Yet Implemented", status::BadRequest)
}

// Forbidden made easy
fn forbidden_call() -> IronResult<Response> {
    api_error(ApiError::M_FORBIDDEN, "One must POST to login", status::Forbidden)
}

// Login API
// URL: _matrix/client/r0/lugin
pub fn login(req: &mut Request) -> IronResult<Response> {
    match req.method {
        Method::Post => {
            not_yet_implemented()
        },
        Method::Get => {
            not_yet_implemented()
        },
        _ => {
            forbidden_call()
        },
    }
}

// URL: _matrix/client/r0/register
//pub fn register(req: &mut Request) -> IronResult<Response> {
//    debug!("Attempting to register an User");
//    match req.method {
//
//    }
//}

// Setup the client routes
pub fn load_client_routes() -> HashMap<String, Box<Handler>> {
    let mut routes: HashMap<String, Box<Handler>> = HashMap::new();
    routes.insert("r0/login".to_string(), Box::new(login));

    routes
}
