use std::mem;

struct City {
  name: String,
}

impl City {
  fn change_name(&mut self, name: &str) {
    let old_name = mem::replace(&mut self.name, name.to_string());
    println!(
      "The city once called {} is now called {}",
      old_name, self.name
    );
  }
}

fn main() {
  let mut capital_city = City {
    name: "Constantinople".to_string(),
  };
  capital_city.change_name("Istanbul");
}
