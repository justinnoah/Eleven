#[derive(Debug, Deserialize, Serialize)]
struct User {
    id: u64,
    name: String,
    password: String,
    email: Option<String>,
}
