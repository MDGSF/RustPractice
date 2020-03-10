use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Viking {
  name: String,
  country: String,
}

impl Viking {
  fn new(name: &str, country: &str) -> Viking {
    Viking {
      name: name.to_string(),
      country: country.to_string(),
    }
  }
}

fn main() {
  let mut vikings = HashMap::new();

  vikings.insert(Viking::new("Einar", "Norway"), 25);
  vikings.insert(Viking::new("Olaf", "Denmark"), 24);
  vikings.insert(Viking::new("Harald", "Iceland"), 12);

  for (viking, health) in &vikings {
    println!("{:?} has {} hp", viking, health);
  }
}
