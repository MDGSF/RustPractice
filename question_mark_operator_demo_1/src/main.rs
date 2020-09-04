fn parse_str(input: &str) -> Result<i32, std::num::ParseIntError> {
  let parsed_number = input.parse::<i32>()?;
  Ok(parsed_number)
}

fn main() {
  let str_vec = vec!["Seven", "8", "9.0", "nice", "6060"];
  for item in str_vec {
    let parsed = parse_str(item);
    println!("{:?}", parsed);
  }
}
