use serde_json::{Value, json};
use axum::{
    Form, Json, response::IntoResponse,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TokenRequest {
    grant_type: String,
    username: String,
    password: String,
}

pub async fn token_handler(Form(payload): Form<TokenRequest>) -> impl IntoResponse {
    println!("Token handler called");
    if payload.grant_type != "password" {
        return Json(json!({ "error": "unsupported_grant_type" }));
    }
    
    if payload.username == "bob" &&  payload.password == "password" {
        Json(json!({ "token": 42 }))
    } else {
        Json(json!({ "error": "invalid_grant" }))
    }
}