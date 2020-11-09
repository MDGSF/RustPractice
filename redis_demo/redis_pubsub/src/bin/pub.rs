extern crate redis;

use anyhow::{anyhow, Context, Result};
use redis::Commands;
use redis::{ControlFlow, PubSubCommands};

fn main() -> Result<()> {
  // connect to redis
  let client = redis::Client::open("redis://127.0.0.1:6379/")?;

  let mut con = client.get_connection()?;
  for i in 0.. {
    con.publish("foo".to_string(), i.to_string())?;
    std::thread::sleep(std::time::Duration::from_secs(1));
  }

  Ok(())
}
