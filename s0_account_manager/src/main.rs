use anyhow::Result;
use s0_account_manager::*;

#[async_std::main]
async fn main() -> Result<()> {
  let config_file = "config.json";
  let config = std::fs::read_to_string(config_file)?;
  let config: Config = serde_json::from_str(&config)?;

  tide::log::start();
  let mut app = tide::with_state(State::new(config.clone()));
  init_routers(&mut app);
  app.listen(config.listener).await?;
  Ok(())
}
