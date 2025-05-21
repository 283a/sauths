use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub iss: String,
}

async fn generate_jwt(username: &str, encoding_key: &EncodingKey) -> String {
    let now = Utc::now();
    let claims = Claims {
        sub: username.to_string(),
        exp: (now + Duration::hours(1)).timestamp() as usize,
        iat: now.timestamp() as usize,
        iss: "sauth_server".to_string(),
    };

    encode(&Header::new(Algorithm::RS256), &claims, encoding_key).unwrap()
}