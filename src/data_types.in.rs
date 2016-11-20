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
