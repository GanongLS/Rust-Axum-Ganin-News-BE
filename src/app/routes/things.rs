use axum::{
  extract::{Path, Query},
  Json,
};
use serde::{de, Deserialize, Deserializer, Serialize};
use std::{fmt, str::FromStr};

#[derive(Serialize)]
pub struct JsonResponse {
  sender: String,
  message: String,
}

pub async fn get_by_id(Path(id): Path<i32>) -> Json<JsonResponse> {
  let response = JsonResponse {
    sender: "Server".into(),
    message: format!("Thank you, your id was: {}", id).into(),
  };
  Json(response)
}

pub async fn get_hard_coded() -> Json<JsonResponse> {
  let response = JsonResponse {
    sender: "Server".into(),
    message: format!("Thank you, your id was being hardcoded to: {}", 19).into(),
  };
  Json(response)
}

/// See the tests below for which combinations of `foo` and `bar` result in
/// which deserializations.
///
/// This example only shows one possible way to do this. [`serde_with`] provides
/// another way. Use which ever method works best for you.
///
/// [`serde_with`]: https://docs.rs/serde_with/1.11.0/serde_with/rust/string_empty_as_none/index.html
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ExampleParams {
  #[serde(default, deserialize_with = "empty_string_as_none")]
  id: Option<u32>,
}

/// Serde deserialization decorator to map empty Strings to None,
fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
  D: Deserializer<'de>,
  T: FromStr,
  T::Err: fmt::Display,
{
  let opt = Option::<String>::deserialize(de)?;
  match opt.as_deref() {
    None | Some("") => Ok(None),
    Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
  }
}

pub async fn get(Query(params): Query<ExampleParams>) -> Json<JsonResponse> {
  match Some(params.id) {
    Some(id) => match id {
      Some(id) => return query_response(id),
      None => return response(),
    },
    None => return response(),
  }

  fn query_response(id: u32) -> Json<JsonResponse> {
    let response = JsonResponse {
      sender: "Server".into(),
      message: format!("Thank you, your id was: {:#?}", id).into(),
    };
    Json(response)
  }

  fn response() -> Json<JsonResponse> {
    let response = JsonResponse {
      sender: "Server".into(),
      message: format!("Thank you, you didn't have any id"),
    };
    Json(response)
  }
}
