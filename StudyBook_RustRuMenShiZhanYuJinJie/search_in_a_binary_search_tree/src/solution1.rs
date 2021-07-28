use binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
  pub fn search_bst(
    root: Option<Rc<RefCell<TreeNode>>>,
    val: i32,
  ) -> Option<Rc<RefCell<TreeNode>>> {
    let mut r = root.clone();
    while let Some(node) = r {
      if node.borrow().val == val {
        return Some(node);
      } else if node.borrow().val > val {
        r = node.borrow().left.clone();
      } else {
        r = node.borrow().right.clone();
      }
    }
    None
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    let input = vec![Some(4), Some(2), Some(7), Some(1), Some(3)];
    let tree = TreeNode::totree(input.clone());
    let result = Solution::search_bst(Some(Rc::new(RefCell::new(tree))), 5);
    assert_eq!(result, None);
  }

  #[test]
  fn test_2() {
    let input = vec![Some(4), Some(2), Some(7), Some(1), Some(3)];
    let tree = TreeNode::totree(input.clone());
    let result = Solution::search_bst(Some(Rc::new(RefCell::new(tree))), 2);

    let input2 = vec![Some(2), Some(1), Some(3)];
    let tree2 = TreeNode::totree(input2.clone());
    assert_eq!(result, Some(Rc::new(RefCell::new(tree2))));
  }
}
