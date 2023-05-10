use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use thiserror::Error;
use validator::ValidationError;

use crate::email::RegistrationConfirmation;

#[derive(Error, Debug)]
pub enum LoginError {
    #[error("Missing Credentials")]
    MissingCredentials,
    #[error("Internal error")]
    InternalError,
    #[error("User was not found or incorrect credentials")]
    NotFound,
    #[error("Invalid token")]
    InvalidToken,
    #[error("Failed to create token")]
    TokenCreation,
    #[error("User registration is unconfirmed.")]
    UnconfirmedUserRegistration,
}

impl IntoResponse for LoginError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "An internal server error occured.",
            ),
            Self::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials."),
            Self::NotFound => (
                StatusCode::NOT_FOUND,
                "User was not found or incorrect credentials.",
            ),
            Self::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token."),
            Self::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create token."),
            Self::UnconfirmedUserRegistration => {
                (StatusCode::CONFLICT, "User registration is unconfirmed.")
            }
        };
        (status, Json(json!({ "error": err_msg }))).into_response()
    }
}

#[derive(Error, Debug)]
pub enum RegistrationError {
    #[error("Invalid Credentials")]
    InvalidCredentials,
    #[error("Internal error")]
    InternalError,
    #[error("User already present.")]
    UserAlreadyTaken,
}

impl IntoResponse for RegistrationError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "An internal server error occured.",
            ),
            Self::InvalidCredentials => (
                StatusCode::BAD_REQUEST,
                "Missing, invalid, or inadequate credentials.",
            ),
            Self::UserAlreadyTaken => (StatusCode::CONFLICT, "User already present."),
        };
        (status, Json(json!({ "error": err_msg }))).into_response()
    }
}

#[derive(Error, Debug)]
pub enum RegistrationConfirmationError {
    #[error("Invalid token")]
    InvalidToken,
    #[error("Internal error")]
    InternalError,
}

impl IntoResponse for RegistrationConfirmationError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token."),
            Self::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "An internal server error occured.",
            ),
        };
        (status, RegistrationConfirmation::failure()).into_response()
    }
}
