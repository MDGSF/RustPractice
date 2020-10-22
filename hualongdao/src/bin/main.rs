use anyhow::Result;
use hualongdao::*;

fn main() -> Result<()> {
  let data = std::fs::read_to_string("levels.json")?;

  let contexts: Vec<InputContext> = serde_json::from_str(&data)?;

  for (i, context) in contexts.iter().enumerate() {
    if i == 1 {
      let mut solution = Solution::new(&context);
      println!("solutin:\n{}", solution);
      solution.process();
      println!("end:\n{}", solution);
      break;
    }
  }

  Ok(())
}
