use crate::errors::AppStateInitializationError;
use diesel_async::AsyncConnection;
use diesel_async::AsyncPgConnection;
use lettre::transport::smtp::authentication::Credentials;
use lettre::SmtpTransport;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    // Put it in an async arc mutex for thread safety
    pub db: Arc<Mutex<AsyncPgConnection>>,
    pub email: SmtpTransport,
}

impl AppState {
    pub async fn new(
        database_url: &str,
        email_relay: &str,
        email: &str,
        password: &str,
    ) -> Result<Self, AppStateInitializationError> {
        let conn = AsyncPgConnection::establish(database_url).await?;
        let credentials = Credentials::new(email.to_owned(), password.to_owned());
        let email = SmtpTransport::relay(email_relay)?
            .credentials(credentials)
            .build();
        let conn = Arc::new(Mutex::new(conn));
        Ok(Self { db: conn, email })
    }
}
