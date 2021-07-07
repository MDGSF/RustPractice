use binary_tree::TreeNode;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
  pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut levels: Vec<Vec<i32>> = vec![];
    if root.is_none() {
      return levels;
    }

    let mut deque: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
    deque.push_back(root);

    while !deque.is_empty() {
      let mut curr_level = vec![];
      let level_length = deque.len();
      for _ in 0..level_length {
        let n = deque.pop_front();
        if let Some(Some(node)) = n {
          curr_level.push(node.borrow().val);
          if node.borrow().left.is_some() {
            deque.push_back(node.borrow().left.clone());
          }
          if node.borrow().right.is_some() {
            deque.push_back(node.borrow().right.clone());
          }
        }
      }
      levels.push(curr_level);
    }

    levels
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    let input = vec![
      Some(1),
      Some(2),
      Some(3),
      Some(4),
      Some(5),
      Some(6),
      Some(7),
    ];
    let tree = TreeNode::totree(input.clone());
    let result = Solution::level_order(Some(Rc::new(RefCell::new(tree))));
    assert_eq!(result, vec![vec![1], vec![2, 3], vec![4, 5, 6, 7]]);
  }

  #[test]
  fn test_2() {
    let input = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
    let tree = TreeNode::totree(input.clone());
    let result = Solution::level_order(Some(Rc::new(RefCell::new(tree))));
    assert_eq!(result, vec![vec![3], vec![9, 20], vec![15, 7]]);
  }
}
