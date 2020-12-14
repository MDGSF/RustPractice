use anyhow::Result;
use s0_account_manager::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
  pub listener: String,
  pub redis_addr: String,
  pub redis_port: u32,
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
  let config_file = "config.json";
  let config = std::fs::read_to_string(config_file)?;
  let config: Config = serde_json::from_str(&config)?;

  tide::log::start();
  let mut app = tide::new();
  init_routers(&mut app);
  app.listen(config.listener).await?;
  Ok(())
}
