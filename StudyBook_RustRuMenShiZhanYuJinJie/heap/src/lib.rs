/*
堆化是指把一个无序数组整理成满足堆特性的堆数组，其过程有从下往上和从上往下两种方式。

1）从下往上的堆化：从索引为1的元素开始，依次取元素的值与其父节点的值比较大小。
如果不满足子节点小于等于父节点，互换两个节点。重复上述过程，直到所有节点间都满足堆的两个特性，

2）从上往下的堆：对索引从 n/2 开始到0的元素，依次取元素的值与其子节点的值比较大小。
如果不满足父节点大于等于子节点，互换两个节点。重复上述过程，
直到所有节点间都满足堆的两个特性。这里之所以从索引n/2开始进行堆化，
是因为 n/2 到 n 的节点都是叶子节点，叶子节点没有子节点，无须堆化操作，
*/

// 从下往上堆化
pub fn build_heap_down_up(nums: &mut Vec<i32>) {
  for i in 1..nums.len() {
    heapify_down_up(nums, i);
  }
}

fn heapify_down_up(nums: &mut Vec<i32>, idx: usize) {
  let mut idx = idx;
  let mut parent_idx = (idx - 1) / 2;
  while nums[idx] > nums[parent_idx] {
    nums.swap(idx, parent_idx);
    idx = parent_idx;
    if idx == 0 {
      break;
    }
    parent_idx = (idx - 1) / 2;
  }
}

// 从上往下堆化
pub fn build_heap_up_down(nums: &mut Vec<i32>) {
  let len = nums.len();
  for i in (0..len / 2).rev() {
    heapify_up_down(nums, i, len);
  }
}

fn heapify_up_down(nums: &mut Vec<i32>, idx: usize, nums_len: usize) {
  let mut idx = idx;
  loop {
    // 将当前节点设置为较大值节点
    let mut max_pos = idx;

    // 判断当前节点是否小于左子节点，如果是则将左子节点设置为较大值节点
    if 2 * idx + 1 < nums_len && nums[idx] < nums[2 * idx + 1] {
      max_pos = 2 * idx + 1;
    }

    // 判断较大值节点是否小于右子节点，如果是则将右子节点设置为较大值节点
    if 2 * idx + 2 < nums_len && nums[max_pos] < nums[2 * idx + 2] {
      max_pos = 2 * idx + 2;
    }

    if max_pos == idx {
      break;
    }

    nums.swap(idx, max_pos);
    idx = max_pos; //继续向下
  }
}

// 在堆中插入元素
pub fn insert(nums: &mut Vec<i32>, x: i32) {
  nums.push(x);
  if nums.len() > 1 {
    // 从下往上堆化
    heapify_down_up(nums, nums.len() - 1);
  }
}

// 删除堆顶元素
pub fn remove_max(nums: &mut Vec<i32>) -> Option<i32> {
  if nums.len() == 0 {
    return None;
  }

  let max_value = nums[0];

  nums[0] = nums[nums.len() - 1];
  nums.remove(nums.len() - 1);

  if nums.len() > 1 {
    // 从上往下堆化
    heapify_up_down(nums, 0, nums.len());
  }

  Some(max_value)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }

  #[test]
  fn test_1() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    build_heap_down_up(&mut nums);

    let mut result = Vec::new();
    while let Some(num) = remove_max(&mut nums) {
      result.push(num);
    }

    assert_eq!(result, vec![7, 6, 5, 4, 3, 2, 1]);
  }

  #[test]
  fn test_2() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    build_heap_up_down(&mut nums);

    let mut result = Vec::new();
    while let Some(num) = remove_max(&mut nums) {
      result.push(num);
    }

    assert_eq!(result, vec![7, 6, 5, 4, 3, 2, 1]);
  }

  #[test]
  fn test_3() {
    let mut nums = vec![];
    for i in 1..=7 {
      insert(&mut nums, i);
    }

    let mut result = Vec::new();
    while let Some(num) = remove_max(&mut nums) {
      result.push(num);
    }

    assert_eq!(result, vec![7, 6, 5, 4, 3, 2, 1]);
  }
}
