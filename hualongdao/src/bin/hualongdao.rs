#[macro_use]
extern crate log;

use anyhow::Result;
use hualongdao::*;

fn main() -> Result<()> {
  init_log(3).unwrap();

  let data = std::fs::read_to_string("levels.json")?;

  let contexts: Vec<InputContext> = serde_json::from_str(&data)?;

  for (i, context) in contexts.iter().enumerate() {
    if i == 17 {
      let mut solution = Solution3::new(&context);
      info!("solutin:\n{}", solution);
      let result = solution.process();
      info!("result = {:?}", result);
      info!("end:\n{}", solution);
      break;
    }
  }

  Ok(())
}
