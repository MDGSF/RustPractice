fn test1() {
  let mut push_string = String::new();
  let mut capacity_counter = 0;
  for _ in 0..100_000 {
    if push_string.capacity() != capacity_counter {
      println!("{}", push_string.capacity());
      capacity_counter = push_string.capacity();
    }
    push_string.push_str("a");
  }
}

fn test2() {
  let mut push_string = String::with_capacity(131072);
  let mut capacity_counter = 0;
  for _ in 0..100_000 {
    if push_string.capacity() != capacity_counter {
      println!("{}", push_string.capacity());
      capacity_counter = push_string.capacity();
    }
    push_string.push_str("a");
  }
}

fn test3() {
  let mut push_string = String::with_capacity(131072);
  let mut capacity_counter = 0;
  for _ in 0..100_000 {
    if push_string.capacity() != capacity_counter {
      println!("{}", push_string.capacity());
      capacity_counter = push_string.capacity();
    }
    push_string.push_str("a");
  }
  push_string.shrink_to_fit();
  println!("{}", push_string.capacity());
  push_string.push('a');
  println!("{}", push_string.capacity());
  push_string.shrink_to_fit();
  println!("{}", push_string.capacity());
}

fn main() {
  test3();
}
