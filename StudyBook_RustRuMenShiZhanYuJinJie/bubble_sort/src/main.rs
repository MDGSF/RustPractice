fn bubble_sort(mut nums: Vec<i32>) -> Vec<i32> {
  if nums.is_empty() {
    return nums;
  }

  for i in 0..nums.len() - 1 {
    // 标记是否发生了元素交换
    let mut flag = false;

    for j in 0..nums.len() - i - 1 {
      if nums[j] > nums[j + 1] {
        nums.swap(j, j + 1);
        flag = true; // 表示有数据交换
      }
    }

    // 没有数据交换，说明已经有序了，就提前退出
    if !flag {
      break;
    }
  }

  nums
}

fn main() {
  let nums = vec![7, 9, 12, 11, 6, 3];
  let result = bubble_sort(nums);
  println!("result = {:?}", result);
}
