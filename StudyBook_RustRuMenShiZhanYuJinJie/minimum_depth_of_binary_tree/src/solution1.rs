use binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
  pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
      Some(node) => {
        if node.borrow().left.is_none() {
          return Self::min_depth(node.borrow().right.clone()) + 1;
        }

        if node.borrow().right.is_none() {
          return Self::min_depth(node.borrow().left.clone()) + 1;
        }

        let left = Self::min_depth(node.borrow().left.clone());
        let right = Self::min_depth(node.borrow().right.clone());
        left.min(right) + 1
      }
      _ => 0,
    }
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    let input = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
    let tree = TreeNode::totree(input.clone());
    let result = Solution::min_depth(Some(Rc::new(RefCell::new(tree))));
    assert_eq!(result, 2);
  }
}
