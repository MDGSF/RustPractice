use std::collections::HashMap;

fn main() {
  let mut map = HashMap::new();
  map.insert("a", 1);
  map.insert("b", 2);
  map.insert("c", 3);

  for (_, val) in map.iter_mut() {
    *val *= 2;
  }

  for (key, val) in &map {
    println!("{}: {}", key, val);
  }
}
