use std::env;

use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm, encode, Header, EncodingKey, errors::Error};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
   pub sub: String,
   pub exp: usize,
}

pub fn encode_jwt(user_id: String) -> Result<String, Error> {
    // Claims is a struct that implements Deserialize
    let claims = Claims{ sub: user_id, exp: 10000000000 };
    let secret = env::var("JWT_SECRET").expect("Missing the JWT_SECRET environment variable.");
    encode(
        &Header::default(), 
        &claims, 
        &EncodingKey::from_secret(secret.as_bytes())
    )
}


pub fn decode_jwt(token: String) -> Option<String> {
    // Claims is a struct that implements Deserialize
    let secret = env::var("JWT_SECRET").expect("Missing the JWT_SECRET environment variable.");
    match decode::<Claims>(
        &token, 
        &DecodingKey::from_secret(secret.as_bytes()), 
        &Validation::new(Algorithm::HS256)
    ) {
        Ok(data) => Some(data.claims.sub),
        Err(_) => None
    }
}