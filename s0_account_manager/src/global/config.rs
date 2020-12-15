use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
  pub listener: String,
  pub redis_addr: String,
  pub redis_port: u32,
}
