extern crate redis;

use anyhow::{anyhow, Context, Result};
use redis::Commands;

fn fetch_an_integer() -> redis::RedisResult<isize> {
  // connect to redis
  let client = redis::Client::open("redis://127.0.0.1:6379/")?;
  let mut con = client.get_connection()?;
  // throw away the result, just make sure it does not fail
  let _: () = con.set("my_key", 42)?;
  // read back the key and return it.  Because the return value
  // from the function is a result for integer this will automatically
  // convert into one.
  con.get("my_key")
}

fn main() -> Result<()> {
  let my_key = fetch_an_integer()?;
  println!("my_key = {}", my_key);

  Ok(())
}
