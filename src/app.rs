mod routes;

use std::time::Duration;

use crate::data::access;
use axum::{error_handling::HandleErrorLayer, http::StatusCode, routing, Router};
use routes::{fallback, hello_world};
use tower::{BoxError, ServiceBuilder};
use tower_http::trace::TraceLayer;

// return Router;
pub async fn app_router() -> Router<()> {
  // init and reading data
  access::print_data().await;

  Router::new()
    .fallback(fallback::fallback)
    .route("/", routing::get(hello_world::get_hello))
    .nest("/books", routes::book_router()) // nested router
    .nest("/articles", routes::article_router()) // nested router
    .merge(routes::tutorial_router()) // merge request
    .layer(
      ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|error: BoxError| async move {
          if error.is::<tower::timeout::error::Elapsed>() {
            Ok(StatusCode::REQUEST_TIMEOUT)
          } else {
            Err((
              StatusCode::INTERNAL_SERVER_ERROR,
              format!("Unhandled internal error: {error}"),
            ))
          }
        }))
        .timeout(Duration::from_secs(60))
        .layer(TraceLayer::new_for_http())
        .into_inner(),
    )
}
