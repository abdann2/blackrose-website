use diesel_async::AsyncPgConnection;
use std::sync::Arc;
use tokio::sync::Mutex;

struct DbInterface {
    // Put it in an async arc mutex for thread safety
    db: Arc<Mutex<AsyncPgConnection>>,
}
