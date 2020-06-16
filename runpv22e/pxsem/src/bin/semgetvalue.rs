use libc;
use runpv22e::*;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "semgetvalue")]
struct Opt {
  #[structopt(name = "name")]
  name: String,
}

fn main() {
  let opt = Opt::from_args();

  let sem = rsem_open(&opt.name, 0, None, None);

  let mut val: libc::c_int = 0;
  rsem_getvalue(sem, &mut val);

  println!("value = {}", val);
}
