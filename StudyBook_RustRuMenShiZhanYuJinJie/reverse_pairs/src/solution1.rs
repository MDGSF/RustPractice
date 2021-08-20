impl Solution {
  pub fn reverse_pairs(mut nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
      return 0;
    }

    let len = nums.len();
    let mut tmp = vec![0; len];
    Self::sort_count(&mut nums, &mut tmp, 0, len - 1) as i32
  }

  fn sort_count(nums: &mut Vec<i32>, tmp: &mut Vec<i32>, left: usize, right: usize) -> usize {
    if left >= right {
      return 0;
    }

    let middle = left + (right - left) / 2;

    let mut count = 0;
    count += Self::sort_count(nums, tmp, left, middle);
    count += Self::sort_count(nums, tmp, middle + 1, right);

    let mut i = left;
    let mut j = middle + 1;
    while i <= middle && j <= right {
      if nums[i] as i64 > 2 * nums[j] as i64 {
        count += middle - i + 1;
        j += 1;
      } else {
        i += 1;
      }
    }

    Self::merge(nums, tmp, left, middle, right);

    count
  }

  fn merge(nums: &mut Vec<i32>, tmp: &mut Vec<i32>, left: usize, middle: usize, right: usize) {
    let mut index = 0;
    let mut i = left;
    let mut j = middle + 1;

    while i <= middle && j <= right {
      if nums[i] <= nums[j] {
        tmp[index] = nums[i];
        index += 1;
        i += 1;
      } else {
        tmp[index] = nums[j];
        index += 1;
        j += 1;
      }
    }

    while i <= middle {
      tmp[index] = nums[i];
      index += 1;
      i += 1;
    }

    while j <= right {
      tmp[index] = nums[j];
      index += 1;
      j += 1;
    }

    for i in left..=right {
      nums[i] = tmp[i - left];
    }
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(Solution::reverse_pairs(vec![1, 3, 2, 3, 1]), 2);
    assert_eq!(Solution::reverse_pairs(vec![2, 4, 3, 5, 1]), 3);
  }
}
