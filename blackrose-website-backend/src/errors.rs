use diesel::ConnectionError as DieselConnectionError;
use lettre::transport::smtp::Error as SmtpError;
use thiserror::Error;

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
