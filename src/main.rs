use axum::{
    Json, Router, extract::FromRequestParts, http::request::Parts,
    routing::get, routing::post,
};
use hyper::StatusCode;
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation, TokenData};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

struct AuthenticatedUser {
    username: String,
}

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .ok_or((
                StatusCode::UNAUTHORIZED,
                "Missing Authorization header".to_string(),
            ))?
            .to_str()
            .map_err(|_| {
                (
                    StatusCode::UNAUTHORIZED,
                    "Invalid Authorization header".to_string(),
                )
            })?;

        if !auth_header.starts_with("Bearer ") {
            return Err((
                StatusCode::UNAUTHORIZED,
                "Invalid Authorization header".to_string(),
            ));
        }
        let token = auth_header.trim_start_matches("Bearer ").trim();

        let secret = "my_secret_key";

        let token_data: TokenData<Claims> = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token".to_string()))?;

        Ok(AuthenticatedUser {
            username: token_data.claims.sub,
        })
    }
}

async fn get_handler() -> Json<serde_json::Value> {
    Json(json!({
        "message": "GET request received"
    }))
}

async fn post_handler(Json(payload): Json<serde_json::Value>) -> Json<serde_json::Value> {
    println!("Received POST payload: {:?}", payload);
    Json(json!({
        "message": "POST request received",
        "data": payload
    }))
}

async fn login_handler(Json(payload): Json<LoginRequest>) -> Json<serde_json::Value> {
    println!("Login attempt: {:?}", payload);

    if payload.username == "bob" && payload.password == "secret" {
        let expiration = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::minutes(5))
            .expect("valid timestamp")
            .timestamp();

        let claims = Claims {
            sub: payload.username,
            exp: expiration as usize,
        };

        let secret = "my_secret_key"; // Replace with your actual secret key

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .unwrap();

        Json(json!({
            "status": "success",
            "access_token": token,
        }))
    } else {
        Json(json!({
            "status": "error",
            "message": "Invalid username or password",
        }))
    }
}

async fn protected(user: AuthenticatedUser) -> Json<serde_json::Value> {
    Json(json!({
        "message": format!("Welcome, {}!", user.username),
    }))
}

fn load_private_key() -> EncodingKey {
    let private_key_pem = fs::read("/keys/private_key.pem").expect("Failed to read private key");
    EncodingKey::from_rsa_pem(&private_key_pem).expect("Invalid RSA private key")
}

fn load_public_key() -> DecodingKey {
    let public_key_pem = fs::read("keys/public_key.pem").expect("Failed to read public key");
    DecodingKey::from_rsa_pem(&public_key_pem).expect("Invalid RSA public key")
}


#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let app = Router::new()
        .route("/get", get(get_handler))
        .route("/post", post(login_handler))
        .route("/protected", get(protected));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
