fn binary_search(nums: &[i32], target: i32) -> Option<usize> {
  let mut left = 0;
  let mut right = nums.len() - 1;
  while left <= right {
    let mid = left + (right - left) / 2;
    if nums[mid] == target {
      return Some(mid);
    } else if nums[mid] < target {
      left = mid + 1;
    } else {
      right = mid - 1;
    }
  }
  None
}

fn main() {
  let nums = vec![1, 2, 3, 4, 5];
  let target = 5;
  let result = binary_search(&nums, target);
  println!("result = {:?}", result);
}
