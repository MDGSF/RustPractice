use binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
  pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = Vec::new();
    Solution::preorder_recursive(root, &mut result);
    result
  }

  fn preorder_recursive(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
    match root {
      Some(node) => {
        result.push(node.borrow().val);
        Solution::preorder_recursive(node.borrow().left.clone(), result);
        Solution::preorder_recursive(node.borrow().right.clone(), result);
      }
      None => return,
    }
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    let input = vec![Some(1), None, Some(2), None, None, Some(3)];
    let tree = TreeNode::totree(input.clone());
    let result = Solution::preorder_traversal(Some(Rc::new(RefCell::new(tree))));
    assert_eq!(result, vec![1, 2, 3]);
  }
}
