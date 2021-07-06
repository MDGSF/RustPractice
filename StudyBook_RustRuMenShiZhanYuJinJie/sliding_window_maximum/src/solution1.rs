use std::collections::VecDeque;

impl Solution {
  pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    if nums.is_empty() || k == 1 {
      return nums;
    }

    let mut result = Vec::new();
    let mut deque = VecDeque::new();
    for i in 0..nums.len() {
      Solution::push(&mut deque, nums[i]);

      if (i as i32) > k - 1 {
        // 让滑动窗口内保持k个元素
        Solution::pop(&mut deque, nums[i - k as usize]);

        // 将最大值加入输入数组
        result.push(Solution::max(&deque));
      } else if (i as i32) == k - 1 {
        // 将前k个元素的最大值加入输入数组
        result.push(Solution::max(&deque));
      }
    }
    result
  }

  fn push(deque: &mut VecDeque<i32>, n: i32) {
    // 当队列不为空且队尾元素小于当前值时，弹出队尾元素
    while !deque.is_empty() && *deque.back().unwrap() < n {
      deque.pop_back();
    }
    deque.push_back(n);
  }

  fn pop(deque: &mut VecDeque<i32>, n: i32) {
    // 当队列不为空且队首元素等于当前值时，弹出队首元素
    if !deque.is_empty() && *deque.front().unwrap() == n {
      deque.pop_front();
    }
  }

  fn max(deque: &VecDeque<i32>) -> i32 {
    // 返回队列中的最大值，即队首元素
    *deque.front().unwrap()
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(
      Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
      vec![3, 3, 5, 5, 6, 7]
    );
  }
}
