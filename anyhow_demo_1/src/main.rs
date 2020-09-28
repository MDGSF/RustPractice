use anyhow::{anyhow, Context, Result};

fn main() -> Result<()> {
  println!("Hello, world!");
  //test1()?;
  test2().context("main test2 failed")?;
  Ok(())
}

fn test1() -> Result<()> {
  Err(anyhow!("test1 anyhow error"))
}

fn test2() -> Result<()> {
  Err(anyhow!("test2 anyhow error"))
}
