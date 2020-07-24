#[macro_use]
extern crate nix;

const ASHMEM_NAME_LEN: usize = 256;
const __ASHMEMIOC: usize = 0x77;

const ASHMEM_SET_NAME: u32 = nix::iow!(__ASHMEMIOC, 1, ASHMEM_NAME_LEN);

fn main() {
  println!("Hello, world!");

  println!("ASHMEM_SET_NAME = {}", ASHMEM_SET_NAME);
}
