// https://zhuanlan.zhihu.com/p/194156624

use std::collections::HashSet;

fn main() {
  let hello = "hello".to_owned();

  let mut items = HashSet::new();
  items.insert(hello.as_str());

  let mut global_set = HashSet::new();
  global_set.insert(hello.as_str());

  while !global_set.is_empty() {
    let mut temp_set = HashSet::new();

    for &item in global_set.iter() {
      let copy = item.to_owned();
      let copy_str = copy.as_str();

      if let Some(inner) = items.get(copy_str).cloned() {
        temp_set.insert(inner);
      }
    }

    std::mem::swap(&mut global_set, &mut temp_set);
    break;
  }
}
