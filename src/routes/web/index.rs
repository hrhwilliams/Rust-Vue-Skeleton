use axum::response::{Html, IntoResponse};

pub async fn index() -> impl IntoResponse {
    Html(include_str!("../../../frontend/dist/index.html"))
}
