impl Solution {
  pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    if nums.len() <= 1 {
      return nums.len() as i32;
    }

    let mut dp = vec![1; nums.len()];
    let mut result = 1;

    for i in 0..nums.len() {
      for j in 0..i {
        if nums[j] < nums[i] {
          dp[i] = dp[i].max(dp[j] + 1);
        }
      }
      result = result.max(dp[i]);
    }

    result
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
  }
}
