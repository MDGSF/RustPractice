impl Solution {
  pub fn move_zeros(nums: &mut Vec<i32>) {
    let mut j = 0;
    for i in 0..nums.len() {
      if nums[i] != 0 {
        nums[j] = nums[i];
        j += 1;
      }
    }

    for k in j..nums.len() {
      nums[k] = 0;
    }
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    let mut nums = vec![0, 1, 0, 3, 12];
    Solution::move_zeros(&mut nums);
    assert_eq!(nums, vec![1, 3, 12, 0, 0]);
  }
}
