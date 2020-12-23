fn main() {
  let helpful_message = if cfg!(windows) { "backslash" } else { "slash" };

  println!("{}", helpful_message);
}
