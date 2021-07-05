use phrases_lib::chinese;
use phrases_lib::english::{farewells, greetings};

fn main() {
  println!("Hello in chinese: {}", chinese::hello());
  println!("Goodbye in chinese: {}", chinese::goodbye());
  println!("Hello in English: {}", greetings::hello());
  println!("Goodbye in English: {}", farewells::goodbye());
}
