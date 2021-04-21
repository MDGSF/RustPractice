use actix_files as fs;
use actix_session::{CookieSession, Session};
use actix_utils::mpsc;
use actix_web::http::{header, Method, StatusCode};
use actix_web::{
  error, get, guard, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Result,
};
use std::env;
use std::io;

#[get("/favicon")]
async fn favicon() -> Result<fs::NamedFile> {
  Ok(fs::NamedFile::open("static/favicon.ico")?)
}

#[get("/welcome")]
async fn welcome(session: Session, req: HttpRequest) -> Result<HttpResponse> {
  println!("{:?}", req);

  let mut counter = 1;
  if let Some(count) = session.get::<i32>("counter")? {
    println!("SESSION value: {}", count);
    counter = count + 1;
  }

  session.set("counter", counter)?;

  Ok(
    HttpResponse::build(StatusCode::OK)
      .content_type("text/html; charset=utf-8")
      .body(include_str!("../static/welcome.html")),
  )
}

async fn with_param(req: HttpRequest, web::Path((name,)): web::Path<(String,)>) -> HttpResponse {
  println!("{:?}", req);

  HttpResponse::Ok()
    .content_type("text/plain")
    .body(format!("Hello {}", name))
}

async fn response_body(path: web::Path<String>) -> HttpResponse {
  let text = format!("Hello {}", *path);

  let (tx, rx_body) = mpsc::channel();
  let _ = tx.send(Ok::<_, Error>(web::Bytes::from(text)));

  HttpResponse::Ok().streaming(rx_body)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
  env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
  env_logger::init();

  HttpServer::new(|| {
    App::new()
      .wrap(CookieSession::signed(&[0; 32]).secure(false))
      .wrap(middleware::Logger::default())
      .service(favicon)
      .service(welcome)
      .service(web::resource("/user/{name}").route(web::get().to(with_param)))
      .service(web::resource("/async-body/{name}").route(web::get().to(response_body)))
      .service(
        web::resource("/test").to(|req: HttpRequest| match *req.method() {
          Method::GET => HttpResponse::Ok(),
          Method::POST => HttpResponse::MethodNotAllowed(),
          _ => HttpResponse::NotFound(),
        }),
      )
  })
  .bind("127.0.0.1:8080")?
  .run()
  .await
}
