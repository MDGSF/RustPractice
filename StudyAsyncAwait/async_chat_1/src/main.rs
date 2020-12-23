mod client;
mod server;

use anyhow::{anyhow, Context, Result};

fn main() -> Result<()> {
  let mut args = std::env::args();
  match (args.nth(1).as_ref().map(String::as_str), args.next()) {
    (Some("client"), None) => client::main(),
    (Some("server"), None) => server::main(),
    _ => Err(anyhow!("Usage: chat [client|server]")),
  }
}
