use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn name_to_hex(name: String) -> String {
  let mut s = DefaultHasher::new();
  name.hash(&mut s);
  let result = s.finish();
  format!("0x{:x}", result)
}

fn main() {
  let result = name_to_hex("huangjian".to_string());
  println!("result = {}", result);
}
