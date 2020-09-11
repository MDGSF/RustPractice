use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() != 2 {
    println!("usage: calc filename");
    std::process::exit(0);
  }

  let mut m: HashMap<u32, u32> = HashMap::new();
  let mut recved_lines = 0;
  let mut valid_numbers = 0;

  if let Ok(lines) = read_lines(&args[1]) {
    for line in lines {
      if let Ok(line) = line {
        recved_lines += 1;
        if recved_lines % 100000 == 0 {
          println!("recved_lines = {}", recved_lines);
        }
        // let v: Vec<&str> = line.rsplit(' ').collect();
        // let key: u32 = v[0].parse().unwrap();
        let key: u32 = line.rsplit(' ').next().unwrap().parse().unwrap();
        *m.entry(key).or_insert(0) += 1;
        if m[&key] == 10 {
          valid_numbers += 1;
          m.remove(&key);
        }
      }
    }
  }

  let valid_lines = valid_numbers * 10;
  let invalid_numbers = m.len();
  let mut invalid_lines = 0;
  for (_key, value) in &m {
    invalid_lines += value;
  }

  let loss_lines = (invalid_numbers * 10) as u32 - invalid_lines;
  let send_lines = recved_lines + loss_lines;

  println!(
    "send lines = {}, recved lines = {}, loss = {}%",
    send_lines,
    recved_lines,
    loss_lines as f64 / send_lines as f64 * 100f64
  );
  println!(
    "Valid Numbers = {}, occupy {} Lines",
    valid_numbers, valid_lines
  );
  println!(
    "Invalid Numbers = {}, occupy {} lines",
    invalid_numbers, invalid_lines
  );
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}
