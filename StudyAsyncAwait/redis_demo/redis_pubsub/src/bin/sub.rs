extern crate redis;

use anyhow::{anyhow, Context, Result};
use redis::Commands;
use redis::{ControlFlow, PubSubCommands};

fn main() -> Result<()> {
  // connect to redis
  let client = redis::Client::open("redis://127.0.0.1:6379/")?;

  let mut con = client.get_connection()?;
  let mut count = 0;
  con.subscribe(&["foo"], |msg| {
    // do something with message
    assert_eq!(msg.get_channel(), Ok(String::from("foo")));

    println!("{:?}", msg);

    // increment messages seen counter
    count += 1;
    match count {
      // stop after receiving 10 messages
      10 => ControlFlow::Break(()),
      _ => ControlFlow::Continue,
    }
  })?;

  Ok(())
}
