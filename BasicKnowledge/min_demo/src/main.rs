fn main() {
  println!("min(1,2,3,4) = {}", 1.min(2).min(3).min(4));

  println!("min(1,2) = {}", std::cmp::min(1, 2));
}
