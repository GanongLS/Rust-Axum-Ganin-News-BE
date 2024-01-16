mod routes;

use crate::data::access;

use axum::{routing, Router};
use routes::{fallback, hello_world, mirror_body_json, mirror_body_string, things};

// return Router;
pub async fn app_router() -> Router<()> {

    // init and reading data
 access::print_data().await;

  Router::new()
    .fallback(fallback::fallback)
    .route("/", routing::get(hello_world::get_hello))
    .route(
      "/mirror_body_string",
      routing::post(mirror_body_string::post_string),
    )
    .route(
      "/mirror_body_json",
      routing::post(mirror_body_json::post_json),
    )
    .route("/things/:id", routing::get(things::get_by_id)) // path variable
    .route("/things/15", routing::get(things::get_hard_coded)) // order doesn't really matter here. karena exhaustive match
    .route("/things", routing::get(things::get)) // query params

  // ada nested route method, tapi belum bisa dipake.
}
