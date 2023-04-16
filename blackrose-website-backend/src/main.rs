use axum::{
    routing::get,
    Router,
};
mod database;
mod handlers;
use handlers::*;
use tokio::main;

#[main]
async fn main() {
    let router = Router::new().route("/", get(root_handler));
    axum::Server::bind(&"127.0.0.1:4000".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}