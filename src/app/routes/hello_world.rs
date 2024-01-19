/// axum handler for "GET /" which returns a string and causes axum to
/// immediately respond with status code `200 OK` and with the string.
pub async fn get_hello() -> String {
  "Hello, Axum!".into()
}
