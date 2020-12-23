extern crate crossbeam;

use crossbeam::crossbeam_channel::unbounded;

fn main() {
  let (s, r) = unbounded();

  s.send("Hello, world!").unwrap();

  println!("{:?}", r.recv());
}
