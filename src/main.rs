use axum::{
    routing::get,
    routing::post,
    Router,
};
mod auth;
use auth::keys::{load_private_key, load_public_key};
mod routes;
use routes::token::token_handler;
use jsonwebtoken::{EncodingKey, DecodingKey};


async fn lol() -> &'static str {
    "LOL"
}

#[tokio::main]
async fn main() {

    println!("Hello, world!");
    let encoding_key = load_private_key();
    let decoding_key = load_public_key();

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/lol", get(lol))
        .route("/token", post(token_handler));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}