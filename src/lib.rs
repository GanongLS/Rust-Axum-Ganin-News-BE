mod app;
mod data;

use tokio::net::TcpListener;

pub async fn run_server() {
  // run our app with hyper, listening globally on port 3000
  let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
  axum::serve(listener, app::app_router().await)
    .await
    .unwrap();
}
