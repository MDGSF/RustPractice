#[derive(PartialEq)]
struct Hostname(String);

fn main() {
  let ordinary_string = String::from("localhost");
  let host = Hostname(ordinary_string);
  if host == ordinary_string {}
}
