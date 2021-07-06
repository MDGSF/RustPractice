use std::collections::HashMap;

impl Solution {
  pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for onestr in strs.iter() {
      let mut chars: Vec<char> = onestr.chars().collect();
      chars.sort();
      let key: String = chars.into_iter().collect();
      let val = map.entry(key).or_insert_with(|| vec![]);
      val.push(onestr.clone());
    }
    map.into_iter().map(|(_key, val)| val).collect()
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {}
}
