use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use diesel::ConnectionError as DieselConnectionError;
use lettre::transport::smtp::Error as SmtpError;
use serde_json::json;
use thiserror::Error;

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
    #[error("Missing Credentials")]
    MissingCredentials,
    #[error("Internal error")]
    InternalError,
    #[error("User already present.")]
    UserAlreadyTaken,
    #[error("Invalid token")]
    InvalidToken,
    #[error("Failed to create token")]
    TokenCreation,
}

impl IntoResponse for RegistrationError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "An internal server error occured.",
            ),
            Self::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials."),
            Self::UserAlreadyTaken => (StatusCode::CONFLICT, "User already present."),
            Self::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token."),
            Self::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create token."),
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
#[derive(Debug, Error)]
pub enum AppStateInitializationError {
    #[error(transparent)]
    DatabaseConnectionError(#[from] DieselConnectionError),
    #[error(transparent)]
    EmailCreationError(#[from] SmtpError),
}

#[derive(Error, Debug)]
pub enum SendEmailError {
    #[error("The provided receiving email `{0}` could not be parsed.")]
    BadReceivingEmail(String),
    #[error("Error rendering html")]
    HtmlRenderingError,
    #[error("Smtp error")]
    SmtpError(#[from] SmtpError),
}
