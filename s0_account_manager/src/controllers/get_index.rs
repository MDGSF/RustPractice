use crate::*;
use tide::{Body, Request};

pub async fn get_index(_req: Request<State>) -> tide::Result<tide::Body> {
  Ok(Body::from_file("views/index.html").await?)
}
