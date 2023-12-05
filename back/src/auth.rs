use rocket::serde::{Serialize, Deserialize, json::Json};

#[derive(Serialize, Deserialize)]
pub struct AuthResponse {
    pub success: bool,
    pub token: String,
}

#[post("/auth")]
pub async fn auth_handler() -> Json<AuthResponse> {
    let response = AuthResponse {
        token: String::from("todo!"),
        success: true,
    };

    Json(response)
}
