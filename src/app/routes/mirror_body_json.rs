use axum::{response, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct MirrorJson {
  sender: String,
  message: String,
}

#[derive(Serialize)]
pub struct JsonResponse {
  sender: String,
  your_message: String,
  message: String,
}

pub async fn post_json(Json(body): Json<MirrorJson>) -> response::Json<JsonResponse> {
  println!("{:#?}", body.sender);
  // let response = body as JsonResponse masih belum tahu caranya nerapin prinsip liskov
  let response = JsonResponse {
    sender: "Server".into(),
    your_message: body.message,
    message: "Thank you".into(),
  };
  Json(response)
  // Json(body)
}
