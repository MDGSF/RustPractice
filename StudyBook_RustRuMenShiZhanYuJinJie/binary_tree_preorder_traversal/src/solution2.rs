use binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
  pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = Vec::new();
    if root.is_none() {
      return result;
    }

    let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
    let mut r = root.clone();

    // 当前节点非空或栈非空时循环
    while r.is_some() || !stack.is_empty() {
      // 若当前节点值非空，访问当前节点值，将当前节点入栈，并进入其左子树访问
      while let Some(node) = r {
        result.push(node.borrow().val);
        stack.push(node.clone());
        r = node.borrow().left.clone();
      }

      // 栈顶节点出栈，并进入其右子树访问
      r = stack.pop();
      if let Some(node) = r {
        r = node.borrow().right.clone();
      }
    }

    result
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
