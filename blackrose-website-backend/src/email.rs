use crate::errors::{AppStateInitializationError, SendEmailError};
use askama::Template;
use axum::response::Html;
use dotenvy::dotenv;
use lettre::{
    message::{header, MultiPart, SinglePart},
    message::{Mailbox, MessageBuilder},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use once_cell::sync::Lazy;
use std::env::var;

const EMAIL_REGISTRATION_CONFIRMATION_SUCCESS_TEXT: &str = "Registration confirmed!";
const EMAIL_REGISTRATION_CONFIRMATION_FAILURE_TEXT: &str = "An internal error has occured.";

pub static EMAIL_RELAY: Lazy<String> = Lazy::new(|| {
    dotenv().expect("No .env file found");
    var("EMAIL_RELAY").expect("Missing EMAIL_RELAY env variable.")
});

pub static EMAIL: Lazy<String> = Lazy::new(|| {
    dotenv().expect("No .env file found");
    var("EMAIL").expect("Missing EMAIL env variable.")
});

pub static EMAIL_PASSWORD: Lazy<String> = Lazy::new(|| {
    dotenv().expect("No .env file found");
    var("EMAIL_PASSWORD").expect("Missing EMAIL_PASSWORD env variable.")
});

const SUBJECT: &str = "Black Rose Community Registration Confirmation";

#[derive(Template)]
#[template(path = "registration_confirmation_email.html")]
struct RegistrationConfirmationEmail<'a> {
    username: &'a str,
    registration_confirmation_link: &'a str,
}

#[derive(Template)]
#[template(path = "registration_confirmation.html")]
pub struct RegistrationConfirmation<'a> {
    registration_confirmation_text: &'a str,
}

impl RegistrationConfirmation<'static> {
    pub fn success() -> Html<String> {
        let html = RegistrationConfirmation {
            registration_confirmation_text: EMAIL_REGISTRATION_CONFIRMATION_SUCCESS_TEXT,
        };

        Html(
            html.render()
                .expect("Failed to render askama template for registration success page."),
        )
    }
    pub fn failure() -> Html<String> {
        let html = RegistrationConfirmation {
            registration_confirmation_text: EMAIL_REGISTRATION_CONFIRMATION_FAILURE_TEXT,
        };

        Html(
            html.render()
                .expect("Failed to render askama template for registration failure page."),
        )
    }
}

#[derive(Clone)]
pub struct EmailClient {
    email: AsyncSmtpTransport<Tokio1Executor>,
}

impl EmailClient {
    pub fn new(
        email_domain: &str,
        email: &str,
        password: &str,
    ) -> Result<Self, AppStateInitializationError> {
        // Ok(Self {
        //     email: SmtpTransport::relay(email_domain)?
        //         .credentials(Credentials::new(email.to_owned(), password.to_owned()))
        //         .build(),
        // })
        Ok(Self {
            email: AsyncSmtpTransport::<Tokio1Executor>::relay(email_domain)?
                .credentials(Credentials::new(email.to_owned(), password.to_owned()))
                .build(),
        })
    }

    /// Send a registration confirmation email given a receiving email, a username, and a registration_confirmation_url to include in the email.
    pub async fn send_registration_confirmation_email(
        &mut self,
        receiving_email: &str,
        username: &str,
        registration_confirmation_url: &str,
    ) -> Result<(), SendEmailError> {
        // Attempt to parse the receiving email string as a mailbox, returning a BadReceivingEmail on failure.
        let receiving_email = receiving_email
            .parse::<Mailbox>()
            .map_err(|_| SendEmailError::BadReceivingEmail(receiving_email.to_owned()))?;
        // Render html version of email
        let html_email_content = RegistrationConfirmationEmail {
            username,
            registration_confirmation_link: registration_confirmation_url,
        };
        // Format plain text version of email
        let text_email_content = format!(
            "Hello {},\n Please click this link to confirm your account registration: {}",
            username, registration_confirmation_url
        );
        // Assemble email message for both plain text and html formats. I don't expect this to fail, so I use the panic method expect.
        let registration_email = REGISTRATION_EMAIL_TEMPLATE
            .to_owned()
            .to(receiving_email)
            .multipart(
                MultiPart::alternative()
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_HTML)
                            .body(
                                html_email_content
                                    .render()
                                    .map_err(|_| SendEmailError::HtmlRenderingError)?,
                            ),
                    )
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_PLAIN)
                            .body(text_email_content),
                    ),
            )
            .expect("Error building email.");
        // Send email
        self.email
            .send(registration_email)
            .await
            .map_err(SendEmailError::SmtpError)?;
        Ok(())
    }
}

static REGISTRATION_EMAIL_TEMPLATE: Lazy<MessageBuilder> = Lazy::new(|| {
    Message::builder()
        .from(EMAIL.parse().unwrap())
        .subject(SUBJECT)
});
