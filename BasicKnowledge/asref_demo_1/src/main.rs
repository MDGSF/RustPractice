use std::fmt::{Debug, Display};

fn print_it<T>(input: T)
where
  T: AsRef<str> + Display + Debug,
{
  println!("{}", input);
}

fn main() {
  print_it("hello");
  print_it("Also, hello".to_string());
  // print_it(8);
}
