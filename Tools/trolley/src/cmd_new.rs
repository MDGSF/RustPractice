use anyhow::Result;
use structopt::StructOpt;

/// Create a new cpp package at <path>
#[derive(StructOpt, Debug, Clone)]
pub struct CommandNew {
  pub path: String,
}

pub struct CommandProcessorNew {
  cmd: CommandNew,
}

impl CommandProcessorNew {
  pub fn new(cmd: CommandNew) -> CommandProcessorNew {
    CommandProcessorNew { cmd }
  }

  pub fn process(&self) -> Result<()> {
    Ok(())
  }
}
