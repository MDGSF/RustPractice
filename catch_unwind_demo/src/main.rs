use std::panic;

fn main() {
  let result = panic::catch_unwind(|| {
    println!("Hello, world!");
  });
  println!("{:?}", result);

  let result = panic::catch_unwind(|| {
    panic!("oh no!");
  });
  println!("{:?}", result);
}
