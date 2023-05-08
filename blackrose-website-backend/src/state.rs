use crate::email::EmailClient;
use crate::errors::AppStateInitializationError;
use diesel_async::AsyncConnection;
use diesel_async::AsyncPgConnection;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    // Put it in an async arc mutex for thread safety
    pub db: Arc<Mutex<AsyncPgConnection>>,
    pub email: EmailClient,
}

impl AppState {
    pub async fn new(
        database_url: &str,
        email_relay: &str,
        email: &str,
        password: &str,
    ) -> Result<Self, AppStateInitializationError> {
        // Establish database connection
        let conn = AsyncPgConnection::establish(database_url).await?;
        // Establish email client
        let email = EmailClient::new(email_relay, email, password)?;
        // Wrap connection in threadsafe mutex
        let conn = Arc::new(Mutex::new(conn));
        Ok(Self { db: conn, email })
    }
}
