fn insertion_sort(mut nums: Vec<i32>) -> Vec<i32> {
  if nums.is_empty() {
    return nums;
  }

  for i in 1..nums.len() {
    let current = nums[i];
    let mut j = (i - 1) as i32;
    while j >= 0 {
      if nums[j as usize] > current {
        nums[(j + 1) as usize] = nums[j as usize];
      } else {
        break;
      }
      j -= 1;
    }
    nums[(j + 1) as usize] = current;
  }

  nums
}

fn main() {
  let nums = vec![7, 9, 12, 11, 6, 3];
  let result = insertion_sort(nums);
  println!("result = {:?}", result);
}
