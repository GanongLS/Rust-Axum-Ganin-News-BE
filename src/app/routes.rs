use axum::{routing, Router};

pub mod articles;
pub mod books;
pub mod fallback;
pub mod hello_world;
pub mod mirror_body_json;
pub mod mirror_body_string;
pub mod things;

pub fn book_router() -> Router {
  Router::new()
    .route("/", routing::get(books::get).put(books::put))
    .route(
      "/:id",
      routing::get(books::get_by_id).delete(books::delete_by_id),
    )
    .route(
      "/:id/form",
      routing::get(books::get_form_by_id).post(books::post_form_by_id),
    )
}

pub fn article_router() -> Router {
  Router::new()
    .route("/", routing::get(articles::get).put(articles::put))
    .route(
      "/:id",
      routing::get(articles::get_by_id).delete(articles::delete_by_id),
    )
    .route(
      "/:id/form",
      routing::get(articles::get_form_by_id).post(articles::post_form_by_id),
    )
}

pub fn tutorial_router() -> Router {
  Router::new()
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
}
