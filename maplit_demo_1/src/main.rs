#[macro_use]
extern crate maplit;

use std::collections::HashMap;

fn test1() {
  let m: HashMap<i32, HashMap<i32, HashMap<i32, i32>>> = HashMap::new();
}

fn test2() {
  let m = hashmap! {
    1 => hashmap!{
      11 => hashmap!{
        111 => 9,
      }
    }
  };
  println!("m = {:?}", m);
}

fn main() {
  test1();
  test2();
}
