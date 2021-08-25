use std::cmp;

pub async fn bubble_sort<T: Copy + cmp::PartialOrd>(nums: &mut Vec<T>) {
  if nums.is_empty() {
    return;
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
}

pub async fn selection_sort<T: cmp::PartialOrd>(nums: &mut Vec<T>) {
  if nums.is_empty() {
    return;
  }

  for i in 0..nums.len() - 1 {
    let mut min_index = i;

    for j in i + 1..nums.len() {
      if nums[j] < nums[min_index] {
        min_index = j;
      }
    }

    if i != min_index {
      nums.swap(i, min_index);
    }
  }
}

pub async fn insertion_sort<T: Copy + cmp::PartialOrd>(arr: &mut Vec<T>) {
  if arr.is_empty() {
    return;
  }

  for i in 1..arr.len() {
    let current = arr[i];

    let mut j = (i - 1) as i32;
    while j >= 0 {
      if arr[j as usize] > current {
        arr[(j + 1) as usize] = arr[j as usize];
      } else {
        break;
      }
      j -= 1;
    }

    arr[(j + 1) as usize] = current;
  }
}

pub async fn heap_sort<T: cmp::PartialOrd>(nums: &mut Vec<T>) {
  build_heap(nums);

  for i in (0..nums.len()).rev() {
    nums.swap(0, i);
    heapify(nums, 0, i);
  }
}

fn build_heap<T: cmp::PartialOrd>(nums: &mut Vec<T>) {
  let len = nums.len();
  for i in (0..len / 2).rev() {
    heapify(nums, i, len);
  }
}

fn heapify<T: cmp::PartialOrd>(nums: &mut Vec<T>, idx: usize, len: usize) {
  let mut idx = idx;
  loop {
    let mut max_pos = idx;
    if 2 * idx + 1 < len && nums[idx] < nums[2 * idx + 1] {
      max_pos = 2 * idx + 1;
    }
    if 2 * idx + 2 < len && nums[max_pos] < nums[2 * idx + 2] {
      max_pos = 2 * idx + 2;
    }

    if max_pos == idx {
      break;
    }
    nums.swap(max_pos, idx);
    idx = max_pos;
  }
}

pub async fn merge_sort<T: Copy + cmp::PartialOrd>(nums: &mut Vec<T>) {
  if nums.is_empty() {
    return;
  }

  let n = nums.len() - 1;
  merge_sort_recursion(nums, 0, n);
}

fn merge_sort_recursion<T: Copy + cmp::PartialOrd>(nums: &mut Vec<T>, left: usize, right: usize) {
  if left >= right {
    return; // 只剩下最后一个元素了
  }

  let middle = left + (right - left) / 2;
  merge_sort_recursion(nums, left, middle);
  merge_sort_recursion(nums, middle + 1, right);

  merge(nums, left, middle, right);
}

fn merge<T: Copy + cmp::PartialOrd>(nums: &mut Vec<T>, left: usize, middle: usize, right: usize) {
  let mut i = left;
  let mut j = middle + 1;
  let mut k = left;
  let mut tmp = vec![];

  while k <= right {
    if i > middle {
      tmp.push(nums[j]);
      j += 1;
      k += 1;
    } else if j > right {
      tmp.push(nums[i]);
      i += 1;
      k += 1;
    } else if nums[i] < nums[j] {
      tmp.push(nums[i]);
      i += 1;
      k += 1;
    } else {
      tmp.push(nums[j]);
      j += 1;
      k += 1;
    }
  }

  for i in 0..=(right - left) {
    nums[left + i] = tmp[i];
  }
}

pub async fn quick_sort<T: cmp::PartialOrd>(nums: &mut Vec<T>) {
  if nums.is_empty() {
    return;
  }

  let len = nums.len();
  quick_sort_recursion(nums, 0, len - 1);
}

fn quick_sort_recursion<T: cmp::PartialOrd>(nums: &mut Vec<T>, left: usize, right: usize) {
  if left >= right {
    return; // 只剩下一个元素
  }

  let pivot = partition(nums, left, right);

  if pivot != 0 {
    quick_sort_recursion(nums, left, pivot - 1);
  }
  quick_sort_recursion(nums, pivot + 1, right);
}

fn partition<T: cmp::PartialOrd>(nums: &mut Vec<T>, left: usize, right: usize) -> usize {
  let pivot = right;

  let mut i = left;
  for j in left..right {
    if nums[j] < nums[pivot] {
      nums.swap(i, j);
      i += 1;
    }
  }

  nums.swap(i, right);

  i
}

