use std::mem;

fn main() {
  println!("Hello, world!");

  let mut x = 5;
  let mut y = 42;

  mem::swap(&mut x, &mut y);

  println!("x = {}", x);
  println!("y = {}", y);
}
