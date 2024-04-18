use std::env;

use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

use rocket::http::Status;
use rocket::request::{Outcome, Request, FromRequest}; // 👈 New!
use rocket::serde::{json::Json, Deserialize, Serialize};

use ts_rs::TS;

#[derive(Responder, Debug)]
pub enum NetworkResponse {
    // #[response(status = 201)]
    // Created(String),
    #[response(status = 400)]
    BadRequest(String),
    #[response(status = 401)]
    Unauthorized(String),
    // #[response(status = 404)]
    // NotFound(String),
    // #[response(status = 409)]
    // Conflict(String),
}

#[derive(Serialize, TS)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub username: String,
    exp: usize,
}

#[derive(Debug)]
pub struct JWT {
    pub claims: Claims,
}

#[derive(Serialize)]
pub enum ResponseBody {
    Message(String),
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub body: ResponseBody,
}

use jsonwebtoken::errors::{Error, ErrorKind};

fn get_secret() -> String {
    "secret".into()
}

pub fn create_jwt(username: String) -> Result<String, Error> {
    let secret = get_secret();

    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::days(7))
        .expect("Invalid timestamp")
        .timestamp();

    let claims = Claims {
        username,
        exp: expiration as usize,
    };

    let header = Header::new(Algorithm::HS512);

    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

fn decode_jwt(token: String) -> Result<Claims, ErrorKind> {
    let secret = get_secret();

    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS512),
    ) {
        Ok(token) => Ok(token.claims),
        Err(err) => Err(err.kind().to_owned()),
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

pub fn login_user(user: Json<LoginRequest>) -> Result<String, NetworkResponse> {
    let correct_username = env::var("ADMIN_USERNAME").unwrap();
    let correct_password = env::var("ADMIN_PASSWORD").unwrap();

    println!("Received username: {}", user.username);
    println!("Received password: {}", user.password);
    println!("Correct username: {}", correct_username);
    println!("Correct password: {}", correct_password);

    if user.username != correct_username || user.password != correct_password {
        return Err(NetworkResponse::Unauthorized(
            "Invalid username or password".to_string(),
        ));
    }

    match create_jwt(user.username.clone()) {
        Ok(token) => Ok(token),
        Err(err) => Err(NetworkResponse::BadRequest(err.to_string())),
    }
}

#[post("/login", format = "application/json", data = "<user>")]
pub fn login_user_handler(user: Json<LoginRequest>) -> Result<String, NetworkResponse> {
    let token = login_user(user)?;

    let response = LoginResponse { token }; 

    Ok(serde_json::to_string(&response).unwrap())
}



#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWT {
    type Error = NetworkResponse;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, NetworkResponse> {
        fn is_valid(key: &str) -> Result<Claims, Error> {
            Ok(decode_jwt(String::from(key))?)
        }

        let auth_header = req.headers().get_one("Authorization");
        let auth_cookie = req.cookies().get("auth");

        let auth_token = match (auth_header, auth_cookie) {
            (Some(header), None) => {
                println!("Authorization header: {}", header);
                Some(header)
            },
            (None, Some(cookie)) => {
                println!("Authorization cookie: {}", cookie.value());
                Some(cookie.value())
            },
            (Some(header), Some(cookie)) => {
                println!("Authorization header: {}", header);
                println!("Authorization cookie: {}", cookie.value());
                println!("Picking authorization header");
                Some(header)
            },
            _ => None,
        };

        match auth_token {
            None => {
                let response = Response { body: ResponseBody::Message(String::from("Error validating JWT token - No token provided"))};
                Outcome::Error((Status::Unauthorized, NetworkResponse::Unauthorized(serde_json::to_string(&response).unwrap()))) 
            },
            Some(key) => match is_valid(key) {
                Ok(claims) => Outcome::Success(JWT {claims}),
                Err(err) => match &err.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                        let response = Response { body: ResponseBody::Message(format!("Error validating JWT token - Expired Token"))};
                        Outcome::Error((Status::Unauthorized, NetworkResponse::Unauthorized(serde_json::to_string(&response).unwrap()))) 
                    },
                    jsonwebtoken::errors::ErrorKind::InvalidToken => {
                        let response = Response { body: ResponseBody::Message(format!("Error validating JWT token - Invalid Token"))};
                        Outcome::Error((Status::Unauthorized, NetworkResponse::Unauthorized(serde_json::to_string(&response).unwrap()))) 
                    },
                    _ => {
                        let response = Response { body: ResponseBody::Message(format!("Error validating JWT token - {}", err))};
                        Outcome::Error((Status::Unauthorized, NetworkResponse::Unauthorized(serde_json::to_string(&response).unwrap()))) 
                    }
                }
            },
        }
    }
}

#[get("/me")]
pub fn auth_me_handler(key: JWT) -> Result<String, NetworkResponse> {
    Ok(format!("You are currently logged in as {}", key.claims.username))
}
