use rand::Rng;

pub struct Solution;

impl Solution {
  pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
    if nums.is_empty() || k > nums.len() as i32 {
      return -1;
    }
    let len = nums.len();
    quick_select(&mut nums, 0, len - 1, len - k as usize)
  }
}

fn quick_select(nums: &mut Vec<i32>, left: usize, right: usize, k_smallest: usize) -> i32 {
  if left == right {
    return nums[left];
  }

  let pivot_index = partition(nums, left, right);

  return if k_smallest == pivot_index {
    nums[k_smallest]
  } else if k_smallest < pivot_index {
    quick_select(nums, left, pivot_index - 1, k_smallest)
  } else {
    quick_select(nums, pivot_index + 1, right, k_smallest)
  };
}

fn partition(nums: &mut Vec<i32>, left: usize, right: usize) -> usize {
  let mut rng = rand::thread_rng();
  let pivot_index = left + rng.gen_range(0, right - left);
  nums.swap(pivot_index, right);
  let pivot = nums[right];

  let mut i = left;
  for j in left..right {
    if nums[j] < pivot {
      nums.swap(i, j);
      i += 1;
    }
  }

  nums.swap(i, right);

  i
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
    assert_eq!(
      Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4),
      4
    );
  }
}
