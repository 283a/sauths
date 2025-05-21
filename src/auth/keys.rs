use jsonwebtoken::{EncodingKey, DecodingKey};
use std::fs;

// TODO: Move this to a config module
//        also consider using a library like `config` or `dotenv` for better configuration managemen
//        and loading environment variables
//        better errror handling
//        

pub fn load_private_key() -> EncodingKey {
    let private_key_pem = fs::read("keys/private_key.pem").expect("Failed to read private key");
    EncodingKey::from_rsa_pem(&private_key_pem).expect("Invalid RSA private key")
}

pub fn load_public_key() -> DecodingKey {
    let public_key_pem = fs::read("keys/public_key.pem").expect("Failed to read public key");
    DecodingKey::from_rsa_pem(&public_key_pem).expect("Invalid RSA public key")
}