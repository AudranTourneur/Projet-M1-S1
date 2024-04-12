use rocket::http::Status;
use rocket::request::{Outcome, Request, FromRequest};

use rocket::serde::{json::Json, Deserialize, Serialize};
use ts_rs::TS;

struct ApiKey<'r>(&'r str);

#[derive(Debug)]
enum ApiKeyError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        /// Returns true if `key` is a valid API key string.
        fn is_valid(key: &str) -> bool {
            // key == "valid_api_key"
            true
        }

        match req.headers().get_one("authorization") {
            None => Outcome::Error((Status::BadRequest, ApiKeyError::Missing)),
            Some(key) if is_valid(key) => Outcome::Success(ApiKey(key)),
            Some(_) => Outcome::Error((Status::BadRequest, ApiKeyError::Invalid)),
        }
    }
}


#[derive(Serialize, Deserialize, TS, Clone)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct LoginRequest {
    pub password: String,
}

#[derive(Serialize, Deserialize, TS, Clone)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct LoginResponse {
    pub token: Option<String>,
}

#[post("/login", format = "json", data = "<input>")]
pub fn login(input: Json<LoginRequest>) -> Json<LoginResponse> {
    Json(LoginResponse {
        token: None,
    })
}