use axum::{
    routing::{get, post},
    Router,
};
mod auth;
mod database;
mod errors;
mod handlers;
mod state;
use crate::state::AppState;
use dotenvy::dotenv;
use handlers::*;
use once_cell::sync::Lazy;
use std::env::var;
use tokio::main;

static KEYS: Lazy<auth::Keys> = Lazy::new(|| {
    dotenv().expect("No .env file found");
    let secret = var("SECRET").expect("Missing SECRET env variable.");
    auth::Keys::new(secret.as_bytes())
});

#[main]
async fn main() {
    // Load .env file
    dotenv().expect("No .env file found");
    let db_url = var("DATABASE_URL").expect("Unable to load DATABASE_URL");
    let email_relay = var("EMAIL_RELAY").expect("Unable to load EMAIL_RELAY");
    let email = var("EMAIL").expect("Unable to load EMAIL");
    let email_password = var("EMAIL_PASSWORD").expect("Unable to load EMAIL_PASSWORD");
    // Establish the database interface
    let db_int = AppState::new(&db_url, &email_relay, &email, &email_password)
        .await
        .expect("Unable to establish connection");

    let router = Router::new()
        .route("/", get(root_handler))
        .route("/login", post(login_handler))
        .route("/register", post(registration_handler))
        .with_state(db_int);
    axum::Server::bind(&"127.0.0.1:4000".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
