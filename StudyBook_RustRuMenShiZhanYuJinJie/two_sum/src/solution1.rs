use std::collections::HashMap;

impl Solution {
  pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = HashMap::new();
    for i in 0..nums.len() {
      let peer = target - nums[i];
      if map.contains_key(&peer) {
        return vec![map[&peer] as i32, i as i32];
      }
      map.insert(nums[i], i);
    }
    vec![]
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
  }
}
