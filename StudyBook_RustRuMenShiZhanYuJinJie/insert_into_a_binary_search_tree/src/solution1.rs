use binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
  pub fn insert_into_bst(
    root: Option<Rc<RefCell<TreeNode>>>,
    val: i32,
  ) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() {
      return Some(Rc::new(RefCell::new(TreeNode::new(val))));
    }
    Solution::insert(&root, val);
    root
  }

  fn insert(root: &Option<Rc<RefCell<TreeNode>>>, val: i32) {
    if let Some(node) = root {
      let mut n = node.borrow_mut();
      let target = if val > n.val {
        &mut n.right
      } else {
        &mut n.left
      };
      if target.is_some() {
        return Solution::insert(target, val);
      }

      *target = Some(Rc::new(RefCell::new(TreeNode::new(val))));
    }
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    let input = vec![Some(4), Some(2), Some(7), Some(1), Some(3)];
    let tree = TreeNode::totree(input.clone());
    let result = Solution::insert_into_bst(Some(Rc::new(RefCell::new(tree))), 5);
    let output = TreeNode::tovec(result);
    assert_eq!(
      output,
      vec![Some(4), Some(2), Some(7), Some(1), Some(3), Some(5), None]
    );
  }
}
