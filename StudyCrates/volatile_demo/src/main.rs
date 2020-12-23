use std::sync::atomic::AtomicI64;
use std::sync::atomic::{AtomicBool, Ordering};
use volatile::Volatile;

struct Test {
  id: Volatile<i64>,
}

static x: AtomicBool = AtomicBool::new(true);

fn main() {
  println!("Hello, world!");

  let atomic_forty_two = AtomicI64::new(42);
  println!("{}", atomic_forty_two.load(Ordering::Relaxed));

  x.store(false, Ordering::Relaxed);
  println!("{}", x.load(Ordering::Relaxed));
}
