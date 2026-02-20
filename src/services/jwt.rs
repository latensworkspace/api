use chrono::Utc;
use dotenvy::dotenv;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
}

pub fn generate_jwt(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    dotenv().ok();

    let secret = std::env::var("JWT_SECRET").unwrap();
    let timestamp = (Utc::now() + chrono::Duration::days(7)).timestamp();

    let claims = Claims {
        sub: user_id.to_string(),
        exp: timestamp,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    );

    Ok(token.unwrap())
}

pub fn verify_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    dotenv().ok();

    let secret = std::env::var("JWT_SECRET").unwrap();

    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;

    Ok(data.claims)
}