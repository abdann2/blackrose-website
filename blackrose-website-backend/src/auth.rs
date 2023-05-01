use crate::{errors::LoginError, KEYS};
use axum::{
    async_trait,
    extract::FromRequestParts,
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    RequestPartsExt, TypedHeader,
};
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// This is the claims struct; it represents the email and the token expiration time in seconds
#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub email: String,
    pub exp: u64,
}

// This is the extractor implementation for Claims, so that we can pull out the claims the request header authorization tag
#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = LoginError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| LoginError::InvalidToken)?;
        let token_data = decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| LoginError::InvalidToken)?;
        Ok(token_data.claims)
    }
}

// Helper function to return the time at which the token will expire
pub fn expire_in_five_hours() -> u64 {
    let now = SystemTime::now();
    let since_epoch = now
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards.");
    let five_hours_ahead = since_epoch + Duration::from_secs(60 * 60 * 5);
    five_hours_ahead.as_secs()
}

// Struct to represent the way we will encode and decode the keys via jsonwebtoken encode and decode functions
pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

// Simple "new" implementation, based on a secret provided by the main control flow
impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}
