use axum::{http::{StatusCode, Uri, }, response::IntoResponse};

/// axum handler for any request that fails to match the router routes.
/// This implementation returns HTTP status code Not Found (404).
pub async fn fallback(uri: Uri) -> impl IntoResponse {
  (
    StatusCode::NOT_FOUND,
    format!("404, No route {}", uri),
  )
}