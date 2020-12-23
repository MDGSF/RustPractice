extern crate redis;

use anyhow::{anyhow, Context, Result};
use redis::Commands;

fn main() -> Result<()> {
  // connect to redis
  let client = match redis::Client::open("redis://127.0.0.1:6379/") {
    Ok(client) => client,
    Err(err) => {
      println!("err = {}", err);
      return Err(anyhow!(err));
    }
  };

  let mut con = client.get_connection()?;
  // throw away the result, just make sure it does not fail
  let _: () = con.set("my_key", 42)?;

  loop {
    let mut con = match client.get_connection() {
      Ok(con) => con,
      Err(err) => {
        println!("err = {}", err);
        continue;
      }
    };

    // read back the key and return it.  Because the return value
    // from the function is a result for integer this will automatically
    // convert into one.
    let result: redis::RedisResult<isize> = con.get("my_key");
    println!("{:?}", result);
    std::thread::sleep(std::time::Duration::from_secs(1));
  }

  //Ok(())
}
