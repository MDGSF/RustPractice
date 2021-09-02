use axum::{
  handler::{get, Handler},
  http::StatusCode,
  response::{Html, IntoResponse},
  Router,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
  if std::env::var("RUST_LOG").is_err() {
    std::env::set_var("RUST_LOG", "global_404_handler=debug");
  }
  tracing_subscriber::fmt::init();

  let app = Router::new().route("/", get(handler));

  let app = app.or(handler_404.into_service());

  let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
  tracing::debug!("listening on {}", addr);
  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}

async fn handler() -> Html<&'static str> {
  Html("<h1>Hello, World!</h1>")
}

async fn handler_404() -> impl IntoResponse {
  (StatusCode::NOT_FOUND, "nothing to see here")
}
