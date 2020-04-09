use std::io::{self, Write};

fn read_input() -> io::Result<()> {
  print!("Please input a string: ");
  io::stdout().flush()?;
  let mut input = String::new();
  let stdin = io::stdin();
  let n = stdin.read_line(&mut input)?;
  println!("{}, {}", n, input.trim());
  Ok(())
}

fn main() {
  read_input();
}
