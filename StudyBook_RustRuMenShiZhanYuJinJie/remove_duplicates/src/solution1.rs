impl Solution {
  pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
      return 0;
    }
    let mut slow = 0;
    for fast in 1..nums.len() {
      if nums[slow] != nums[fast] {
        nums[slow + 1] = nums[fast];
        slow += 1;
      }
    }
    (slow + 1) as i32
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    let mut nums = vec![1, 1, 2];
    let result = Solution::remove_duplicates(&mut nums);
    assert_eq!(nums[..result as usize], [1, 2]);
    assert_eq!(result, 2);
  }

  #[test]
  fn test_2() {
    let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    let result = Solution::remove_duplicates(&mut nums);
    assert_eq!(nums[..result as usize], [0, 1, 2, 3, 4]);
    assert_eq!(result, 5);
  }
}
