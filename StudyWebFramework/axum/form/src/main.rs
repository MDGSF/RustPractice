use axum::{extract::Form, handler::get, response::Html, Router};
use serde::Deserialize;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
  if std::env::var("RUST_LOG").is_err() {
    std::env::set_var("RUST_LOG", "form=debug");
  }
  tracing_subscriber::fmt::init();

  let app = Router::new().route("/", get(show_form).post(accept_form));

  let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
  tracing::debug!("listening on {}", addr);
  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}

async fn show_form() -> Html<&'static str> {
  Html(
    r#"
<!doctype html>
<html>
  <head></head>
  <body>
    <form action="/" method="post">
      <label for="name">
        Enter your name:
        <input type="text" name="name">
      </label>

      <label>
        Enter your email:
        <input type="text" name="email">
      </label>

      <input type="submit" value="Subscribe!">
    </form>
  </body>
</html>
    "#,
  )
}

#[derive(Deserialize, Debug)]
struct Input {
  name: String,
  email: String,
}

async fn accept_form(Form(input): Form<Input>) {
  dbg!(&input);
}
