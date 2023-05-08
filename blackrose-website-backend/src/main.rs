#![feature(async_closure)]
use axum::{
    routing::{get, post},
    Router,
};
mod auth;
mod database;
mod email;
mod errors;
mod handlers;
mod state;
mod utils;
use crate::email::{EMAIL, EMAIL_PASSWORD, EMAIL_RELAY};
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

static DB_URL: Lazy<String> = Lazy::new(|| {
    dotenv().expect("No .env file found");
    var("DATABASE_URL").expect("Missing DATABASE_URL env variable.")
});

static BASE_URL: Lazy<String> = Lazy::new(|| {
    dotenv().expect("No .env file found");
    var("BASE_URL").expect("Missing BASE_URL env variable.")
});

#[main]
async fn main() {
    // Establish the database interface
    let mut db_int = AppState::new(&DB_URL, &EMAIL_RELAY, &EMAIL, &EMAIL_PASSWORD)
        .await
        .expect("Unable to establish connection");

    let router = Router::new()
        .route("/", get(root_handler))
        .route("/login", post(login_handler))
        .route("/register", post(registration_handler))
        .route("/register/confirm", get(registration_confirmation_handler))
        .with_state(db_int);
    axum::Server::bind(&"127.0.0.1:4000".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
