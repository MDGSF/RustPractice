use libc;
use pxsem::*;
use runpv22e::*;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "semcreate")]
struct Opt {
  #[structopt(short = "e")]
  exclusive: bool,

  /// initial value for semaphore
  #[structopt(short = "i", default_value = "1")]
  initalvalue: u32,

  #[structopt(name = "name")]
  name: String,
}

fn main() {
  let opt = Opt::from_args();

  let mut flags: i32 = libc::O_RDWR | libc::O_CREAT;
  if opt.exclusive {
    flags |= libc::O_EXCL;
  }

  let sem = rsem_open(
    &opt.name,
    flags,
    Some(FILE_MODE),
    Some(opt.initalvalue),
  );

  rsem_close(sem);
}
