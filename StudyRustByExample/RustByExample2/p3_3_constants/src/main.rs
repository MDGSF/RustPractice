static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
  n > THRESHOLD
}

fn main() {
  println!("This is {}", LANGUAGE);
  println!("The Threshold is {}", THRESHOLD);

  let n = 16;
  println!("{} is {}", n, if is_big(n) { "big" } else { "small" });
}
