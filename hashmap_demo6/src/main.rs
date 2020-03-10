use std::collections::HashMap;

fn main() {
  let mut map: HashMap<i32, i32> = (0..8).map(|x| (x, x * 10)).collect();
  map.retain(|&k, _| k % 2 == 0); // 过滤出 key 为偶数的
  println!("{:?}", map);
}
