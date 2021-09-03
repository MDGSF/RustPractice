use axum::{
  extract::{ContentLengthLimit, Multipart},
  handler::get,
  response::Html,
  Router,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
  if std::env::var("RUST_LOG").is_err() {
    std::env::set_var("RUST_LOG", "multipart_form=debug,tower_http=debug");
  }
  tracing_subscriber::fmt::init();

  let app = Router::new()
    .route("/", get(show_form).post(accept_form))
    .layer(tower_http::trace::TraceLayer::new_for_http());

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
    <form action="/" method="post" enctype="multipart/form-data">
      <label>
        Upload file:
        <input type="file" name="file" multiple>
      </label>

      <input type="submit" value="Upload files">
    </form>
  </body>
</html>
    "#,
  )
}

async fn accept_form(
  ContentLengthLimit(mut multipart): ContentLengthLimit<
    Multipart,
    { 250 * 1024 * 1024 },
  >,
) {
  while let Some(field) = multipart.next_field().await.unwrap() {
    let name = field.name().unwrap().to_string();
    let data = field.bytes().await.unwrap();

    println!("Length of `{}` is {} bytes.", name, data.len());
  }
}
