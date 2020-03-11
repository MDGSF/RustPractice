// use async_std::prelude::*;
use cookie::Cookie;
use serde::{Deserialize, Serialize};
use tide::{Request, Response};

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
  let mut app = tide::new();

  app.middleware(tide::middleware::RequestLogger::new());

  app.at("/").get(|_| async move { "Hello, World!" });

  app.at("/echo").get(|req| async move { req });

  app.at("/hello").get(hello);

  app
    .at("/hello/:name")
    .get(|_| async move { "Hello, World!" });

  app
    .at("/count")
    .post(|mut req: Request<()>| async move {
      let mut counter: Counter = req.body_json().await.unwrap();
      println!("count is {}", counter.count);
      counter.count += 1;
      Response::new(200).body_json(&counter).unwrap()
    });

  app.at("fib/:n").get(fibCtrl);

  let mut inner_gates = tide::new();
  inner_gates
    .at("/")
    .get(|_| async move { "This is an area in front of gates" });
  inner_gates
    .at("/open")
    .get(|_| async move { "Open the gates!" });
  inner_gates
    .at("/close")
    .get(|_| async move { "Close the gates!" });
  app.at("/gates").nest(inner_gates);

  app.at("/echo/string").post(echo_string);
  // app.at("/echo/bytes").post(echo_bytes);

  let mut inner_cookie = tide::new();
  inner_cookie.at("/").get(retrieve_cookie);
  inner_cookie.at("/set").get(set_cookie);
  inner_cookie.at("/remove").get(remove_cookie);
  app.at("/cookie").nest(inner_cookie);

  app.listen("127.0.0.1:8080").await?;
  Ok(())
}

async fn hello(_req: Request<()>) -> String {
  String::from("hello")
}

#[derive(Serialize, Deserialize, Debug)]
struct Counter {
  count: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Message {
  author: Option<String>,
  contents: String,
}

async fn echo_string(mut req: Request<()>) -> String {
  let msg = req.body_string().await.unwrap();
  println!("String: {}", msg);
  msg
}

async fn echo_bytes(mut req: Request<()>) -> Vec<u8> {
  let msg = req.body_bytes().await.unwrap();
  println!("Bytes: {:?}", msg);
  msg
}

async fn retrieve_cookie(req: Request<()>) -> String {
  format!("hello cookies: {:?}", req.cookie("hello").unwrap())
}

async fn set_cookie(_req: Request<()>) -> Response {
  let mut result = Response::new(200);
  result.set_cookie(Cookie::new("hello", "world"));
  result
}

async fn remove_cookie(_req: Request<()>) -> Response {
  let mut result = Response::new(200);
  result.remove_cookie(Cookie::named("hello"));
  result
}

fn fib(n: usize) -> usize {
  if n < 2 {
    return n;
  }
  let mut f1 = 0;
  let mut f2 = 1;
  for _ in 2..=n {
    let f3 = f1 + f2;
    f1 = f2;
    f2 = f3;
  }
  return f2;
}

async fn fibCtrl(req: Request<()>) -> String {
  use std::time::Instant;
  let n: usize = req.param("n").unwrap_or(0);
  let start = Instant::now();
  let result = fib(n);
  let duration = Instant::now().duration_since(start).as_millis();
  format!(
    "The fib of {} is {}. It was computed in {} milliseconds.\n",
    n, result, duration,
  )
}
