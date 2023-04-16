use axum::{
    extract::{Json, Path, Query},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
mod database;
// use std::sync::Arc;
use tokio::main;
// struct AppState {}

#[main]
async fn main() {
    let router = Router::new().route("/", get(root_handler));
    axum::Server::bind(&"127.0.0.1:4000".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}

async fn root_handler() -> impl IntoResponse {
    Html(include_str!("../../blackrose-website-frontend/index.html"))
}
