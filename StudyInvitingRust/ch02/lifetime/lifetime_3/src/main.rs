fn return_str<'a>() -> &'a str {
  let mut s = "Rust".to_string();
  for _i in 0..3 {
    s.push_str("Good ");
  }
  &s[..]
}

fn main() {
  let x = return_str();
}
