use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// change string to hex string.
pub fn str_to_hex(name: String) -> String {
  let mut s = DefaultHasher::new();
  name.hash(&mut s);
  let result = s.finish();
  format!("0x{:x}", result)
}

/// change hex string to i32.
pub fn hex_to_i32(key: &str) -> i32 {
  let mut without_prefix_key = key.trim_start_matches("0x");
  if without_prefix_key.len() > 5 {
    without_prefix_key = &without_prefix_key[..5];
  }
  let shm_key = i32::from_str_radix(without_prefix_key, 16).unwrap();
  return shm_key;
}

/// map topic to UDP multicast address.
/// topic -> i32 (0 ~ 1000000) -> multicast address
/// multicast address: 224.100.100.100 ~ 224.200.200.200
pub fn str_to_multicast(name: &str) -> String {
  let hex_str = str_to_hex(name.to_string());
  let num = hex_to_i32(&hex_str);
  let second = num / 10000 + 100;
  let third = num % 10000 / 100 + 100;
  let fourth = num % 100 + 100;
  let result = format!("224.{}.{}.{}", second, third, fourth);
  result
}

fn test(name: &str) {
  let result = str_to_multicast(name);
  println!("{: <25} -> {:?}", name, result);
}

fn main() {
  test("t1");
  test("t2");
  test("t3");
  test("t4");
  test("t5");
  test("testaa");
  test("adas");
  test("/adas");
  test("/adas/vehicle");
  test("/adas/lane");
  test("/adas/lldw");
  test("/adas/rldw");
  test("/adas/hmw");
  test("/adas/fcw");
  test("dms");
  test("/dms");
  test("/dms/eyeclose");
  test("/dms/yawn");
  test("/dms/look_around");
  test("/dms/phone");
  test("/dms/smoking");
  test("/dms/look_down");
  test("/dms/absence");
  test("/dms/driver_change");
  test("/dms/look_up");
}
