mod routes;

use crate::data::access;
use axum::{routing, Router};
use routes::{fallback, hello_world};

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
}
