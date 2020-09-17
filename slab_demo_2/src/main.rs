extern crate slab;

use slab::Slab;

fn main() {
  let mut slab = Slab::new();
  let mut capacity = slab.capacity();
  for i in 0..10000 {
    slab.insert(i);
    if capacity != slab.capacity() {
      println!("len = {}, capacity = {}", slab.len(), slab.capacity());
      capacity = slab.capacity();
    }
  }
}
