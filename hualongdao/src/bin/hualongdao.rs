#[macro_use]
extern crate log;

use anyhow::Result;
use hualongdao::*;

fn main() -> Result<()> {
  init_log(3).unwrap();

  let data = std::fs::read_to_string("levels.json")?;

  let contexts: Vec<InputContext> = serde_json::from_str(&data)?;

  for (i, context) in contexts.iter().enumerate() {
    if i == 1 {
      let mut solution = Solution::new(&context);
      info!("solutin:\n{}", solution);
      solution.process();
      info!("end:\n{}", solution);
      break;
    }
  }

  Ok(())
}
