use binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
  pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = Vec::new();
    if root.is_none() {
      return result;
    }

    let mut stack1: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
    let mut stack2: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
    stack1.push(root);

    while let Some(Some(node)) = stack1.pop() {
      if node.borrow().left.is_some() {
        stack1.push(node.borrow().left.clone());
      }
      if node.borrow().right.is_some() {
        stack1.push(node.borrow().right.clone());
      }
      stack2.push(Some(node));
    }

    while let Some(Some(node)) = stack2.pop() {
      result.push(node.borrow().val);
    }

    result
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
    let result = Solution::postorder_traversal(Some(Rc::new(RefCell::new(tree))));
    assert_eq!(result, vec![4, 5, 2, 6, 7, 3, 1]);
  }

  #[test]
  fn test_2() {
    let input = vec![Some(1), None, Some(2), None, None, Some(3)];
    let tree = TreeNode::totree(input.clone());
    let result = Solution::postorder_traversal(Some(Rc::new(RefCell::new(tree))));
    assert_eq!(result, vec![3, 2, 1]);
  }
}
