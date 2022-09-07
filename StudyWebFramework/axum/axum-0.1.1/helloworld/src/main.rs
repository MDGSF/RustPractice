use axum::prelude::*;

#[tokio::main]
async fn main() {
  let app = route("/", get(|| async { "Hello, World!" }));

  hyper::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
}
