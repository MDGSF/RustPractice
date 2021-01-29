fn foo<'a>(x: &'a str, y: &'a str) -> &'a str {
  let result = String::from("really long string");
  result.as_str()
}

fn main() {
  let x = "hello";
  let y = "rust";
  foo(x, y);
}
