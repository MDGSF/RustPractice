use askama::Template;
use axum::{
  body::{Bytes, Full},
  extract,
  handler::get,
  http::{Response, StatusCode},
  response::{Html, IntoResponse},
  Router,
};
use std::{convert::Infallible, net::SocketAddr};

#[tokio::main]
async fn main() {
  if std::env::var("RUST_LOG").is_err() {
    std::env::set_var("RUST_LOG", "templates=debug");
  }
  tracing_subscriber::fmt::init();

  let app = Router::new().route("/greet/:name", get(greet));

  let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
  tracing::debug!("listening on {}", addr);
  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}

async fn greet(
  extract::Path(name): extract::Path<String>,
) -> impl IntoResponse {
  let template = HelloTemplate { name };
  HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate {
  name: String,
}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
  T: Template,
{
  type Body = Full<Bytes>;
  type BodyError = Infallible;

  fn into_response(self) -> Response<Self::Body> {
    match self.0.render() {
      Ok(html) => Html(html).into_response(),
      Err(err) => Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Full::from(format!(
          "Failed to render template. Error: {}",
          err
        )))
        .unwrap(),
    }
  }
}
