use anyhow::Result;
use structopt::StructOpt;
use trolley::*;

/// Cpp's package manager
#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "trolley")]
struct Opt {
  #[structopt(subcommand)]
  cmd: Command,
}

#[derive(StructOpt, Debug, Clone)]
enum Command {
  /// Create a new cpp package
  New(CommandNew),
}

fn main() -> Result<()> {
  let opt = Opt::from_args();

  match opt.cmd {
    Command::New(cmd) => CommandProcessorNew::new(cmd).process()?,
  }

  Ok(())
}
