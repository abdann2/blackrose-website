use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest},
    http::Request,
    Json,
};
use serde::Deserialize;
use validator::{Validate, ValidationError};
use zxcvbn::zxcvbn;

use super::errors::RegistrationError;

#[derive(Deserialize)]
pub struct UserCredentials {
    pub email: String,
    pub password: String,
}

/// This validates a password by checking it against a million common passwords, erroring if the password is too simple, common, short, or contains non-ascii characters.
fn validate_password(password: &str) -> Result<(), ValidationError> {
    match zxcvbn(password, &[]) {
        Ok(entropy) => {
            if entropy.score() <= 3 {
                Err(ValidationError::new("Weak password."))
            } else {
                Ok(())
            }
        }
        Err(_) => Err(ValidationError::new("Password validation error.")),
    }
}

#[derive(Deserialize, Validate)]
pub struct UserRegistrationCredentials {
    pub username: String,
    pub display_name: String,
    #[validate(email)]
    pub email: String,
    // This should automatically check password length. It should then call validate_password for further password validation
    #[validate(length(min = 10, max = 100), custom = "validate_password")]
    pub password: String,
}

#[async_trait]
impl<S, B> FromRequest<S, B> for UserRegistrationCredentials
where
    S: Send + Sync,
    B: Send + 'static,
    // This last bound is required so that we can take the request and directly transform it into a Json<UserRegistrationCredentials> without having to copy the parsing implementation of axum::Json<T>
    Json<UserRegistrationCredentials>: FromRequest<S, B, Rejection = JsonRejection>,
{
    type Rejection = RegistrationError;
    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        match Json::<Self>::from_request(req, state).await {
            Ok(value) => {
                // Do the validation here and propagate the error as "InvalidCredentials"
                value
                    .validate()
                    .map_err(|_| Self::Rejection::InvalidCredentials)?;
                // Credentials should be validated now, return ok
                Ok(value.0)
            }
            Err(_) => Err(Self::Rejection::InternalError),
        }
    }
}

#[derive(Deserialize)]
pub struct RegistrationTokenQueryExtractor {
    pub registration_token: String,
}
