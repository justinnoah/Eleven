#[allow(non_camel_case_types, dead_code)]
#[derive(Debug, Serialize)]
pub enum ApiError {
    M_FORBIDDEN,
    // Evuntually the following will be used and the allow dead_code can be removed
    M_UNKNOWN_TOKEN,
    M_BAD_JSON,
    M_NOT_JSON,
    M_NOT_FOUND,
    M_LIMIT_EXCEEDED,
    M_USER_IN_USE,
    M_INVALID_USERNAME,
    M_ROOM_IN_USE,
    M_BAD_PAGINATION,
    M_THREEPID_IN_USE,
    M_THREEPID_NOT_FOUND,
    M_THREEPID_NOT_TRUSTED,
    // Unofficial?
    M_NOT_IMPLEMENTED,
}

#[derive(Debug, Serialize)]
pub struct ApiErrorResponse {
    pub errorcode: ApiError,
    pub error: String,
}

#[derive(Debug, Deserialize)]
struct LoginRequest {
    address: Option<String>,
    medium: Option<String>,
    password: String,
    type_: String,
    user: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
struct LoginResponse {
    access_token: String,
    home_server: String,
    user_id: String,
    refresh_token: Option<String>,
}
