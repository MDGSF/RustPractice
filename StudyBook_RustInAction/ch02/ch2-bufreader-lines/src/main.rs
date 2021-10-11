use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
  let f = File::open("Cargo.toml").unwrap();
  let reader = BufReader::new(f);
  for line in reader.lines() {
    let line = line.unwrap();
    println!("{} ({} bytes long)", line, line.len());
  }
}
