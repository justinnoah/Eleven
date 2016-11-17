use std::error::Error as StdError;
use std::fmt::{self, Display};

use jsonway::{ObjectBuilder, ObjectSerializer};
pub use super::super::errors::ApiError;


#[derive(Debug)]
pub struct JSONApiError {
    errorcode: String,
    error: String,
}

impl JSONApiError {
    pub fn new(error: ApiError, message: String, namespace: String) -> JSONApiError {
        JSONApiError {
            errorcode: format!("{n}_{e:?}", n = namespace.to_uppercase(), e = error),
            error: message,
        }
    }
}

impl Display for JSONApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl StdError for JSONApiError {
    fn description<'a>(&self) -> &'a str {
        return "JSONApiError";
    }
}

pub struct ApiErrorSerializer;
impl ObjectSerializer<JSONApiError> for ApiErrorSerializer {
    fn root(&self) -> Option<&str> { None }
    fn build(&self, err: &JSONApiError, json: &mut ObjectBuilder) {
        json.set("errorcode", err.errorcode.clone());
        json.set("error", err.error.clone());
    }
}
