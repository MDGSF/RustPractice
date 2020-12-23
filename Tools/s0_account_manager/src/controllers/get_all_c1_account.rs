extern crate redis;

use crate::*;
use redis::Commands;
use serde::{Deserialize, Serialize};
use tide::{Body, Request};

#[derive(Deserialize, Serialize)]
struct RespData {
  message: String,
  accounts: Vec<String>,
  numbers: usize,
}

pub async fn get_all_c1_account(req: Request<State>) -> tide::Result<tide::Body> {
  let state = req.state();

  // connect to redis
  let redis_config = format!(
    "redis://{}:{}/",
    state.config.redis_addr, state.config.redis_port
  );
  let client = redis::Client::open(redis_config)?;
  let mut con = client.get_connection()?;
  let accounts: Vec<String> = con.hkeys("c1_auth_keys")?;

  let numbers = accounts.len();
  let resp = RespData {
    message: "get all c1 accounts success".to_string(),
    accounts,
    numbers,
  };
  Ok(Body::from_json(&resp)?)
}
