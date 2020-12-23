extern crate slab;

use slab::Slab;

fn main() {
  let mut slab = Slab::new();

  let hello = slab.insert("hello");
  let world = slab.insert("world");

  println!("{:?}", slab[hello]);
  println!("{:?}", slab[world]);

  slab[world] = "earth";
  println!("{:?}", slab[world]);
}
