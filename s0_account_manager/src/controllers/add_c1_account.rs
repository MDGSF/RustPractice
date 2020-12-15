extern crate redis;

use crate::*;
use redis::Commands;
use serde::{Deserialize, Serialize};
use tide::{Body, Request};

#[derive(Deserialize, Serialize)]
struct ReqData {
  name: String,
  key: String,
}

#[derive(Deserialize, Serialize)]
struct RespData {
  message: String,
}

pub async fn add_c1_account(mut req: Request<State>) -> tide::Result<tide::Body> {
  let req_data: ReqData = req.body_json().await?;

  let state = req.state();

  // connect to redis
  let redis_config = format!(
    "redis://{}:{}/",
    state.config.redis_addr, state.config.redis_port
  );
  let client = redis::Client::open(redis_config)?;
  let mut con = client.get_connection()?;
  let _: () = con.hset("c1_auth_keys", req_data.name, req_data.key)?;

  let resp = RespData {
    message: "add c1 account success".to_string(),
  };
  Ok(Body::from_json(&resp)?)
}
