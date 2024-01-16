use ganin_news as GNewsApp;

#[tokio::main]
async fn main() {
  GNewsApp::run_server().await;
}
