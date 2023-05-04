use diesel::ConnectionError;
use diesel_async::AsyncConnection;
use diesel_async::AsyncPgConnection;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    // Put it in an async arc mutex for thread safety
    pub db: Arc<Mutex<AsyncPgConnection>>,
    pub email: 
}

impl AppState {
    pub async fn new(database_url: &str) -> Result<Self, ConnectionError> {
        let conn = AsyncPgConnection::establish(database_url).await?;
        let conn = Arc::new(Mutex::new(conn));
        Ok(Self { db: conn })
    }
}
