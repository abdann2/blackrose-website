use axum::response::{Html, IntoResponse, Response};

pub async fn root_handler() -> Response {
    Html(include_str!("../../blackrose-website-frontend/index.html")).into_response()
}
