use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None,
    }
  }

  pub fn totree(vals: Vec<Option<i32>>) -> Self {
    TreeNode::totree_recursive(&vals, 0).unwrap()
  }

  fn totree_recursive(vals: &Vec<Option<i32>>, idx: usize) -> Option<TreeNode> {
    if idx >= vals.len() {
      return None;
    }
    if vals[idx].is_none() {
      return None;
    }

    let mut node = TreeNode::new(vals[idx].unwrap());

    let left = TreeNode::totree_recursive(vals, 2 * idx + 1);
    if let Some(left) = left {
      node.left = Some(Rc::new(RefCell::new(left)));
    }

    let right = TreeNode::totree_recursive(vals, 2 * idx + 2);
    if let Some(right) = right {
      node.right = Some(Rc::new(RefCell::new(right)));
    }

    Some(node)
  }

  pub fn tovec(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
    let mut result = Vec::new();
    if root.is_none() {
      return result;
    }

    let mut deque = VecDeque::new();
    let mut level = VecDeque::new();
    level.push_back(root);
    deque.push_back(level);

    while !deque.is_empty() {
      let mut level = deque.pop_front().unwrap();
      let mut next_level = VecDeque::new();
      let mut has_val = false;
      for i in 0..level.len() {
        let node = level[i].take();
        match node {
          Some(node) => {
            has_val = true;
            next_level.push_back(node.borrow_mut().left.take());
            next_level.push_back(node.borrow_mut().right.take());
            level[i] = Some(node);
          }
          None => {
            next_level.push_back(None);
            next_level.push_back(None);
          }
        }
      }
      if has_val {
        for i in 0..level.len() {
          let node = level[i].take();
          match node {
            Some(node) => {
              result.push(Some(node.borrow().val));
            }
            None => {
              result.push(None);
            }
          }
        }

        deque.push_back(next_level);
      }
    }

    result
  }
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

    let output = TreeNode::tovec(Some(Rc::new(RefCell::new(tree))));

    assert_eq!(output, input);
  }

  #[test]
  fn test_2() {
    let input = vec![Some(1), Some(2), Some(3), Some(4), None, Some(6), None];

    let tree = TreeNode::totree(input.clone());

    let output = TreeNode::tovec(Some(Rc::new(RefCell::new(tree))));

    assert_eq!(output, input);
  }

  #[test]
  fn test_3() {
    let input = vec![Some(1), Some(2), None, Some(4), None, None, None, Some(8)];

    let tree = TreeNode::totree(input.clone());

    let output = TreeNode::tovec(Some(Rc::new(RefCell::new(tree))));

    assert_eq!(
      output,
      vec![
        Some(1),
        Some(2),
        None,
        Some(4),
        None,
        None,
        None,
        Some(8),
        None,
        None,
        None,
        None,
        None,
        None,
        None
      ]
    );
  }
}
