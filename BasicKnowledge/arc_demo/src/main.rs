use std::sync::Arc;
use std::thread;

fn main() {
  let numbers: Vec<_> = (0..10u32).collect();
  println!("numbers = {:?}", numbers);

  let shared_numbers = Arc::new(numbers);
  println!("shared_numbers = {:?}", shared_numbers);

  for _ in 0..10 {
    let child_numbers = shared_numbers.clone();

    thread::spawn(move || {
      println!("child_numbers = {:?}", child_numbers);
    });
  }
}
