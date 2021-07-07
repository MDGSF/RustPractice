// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>,
}

impl ListNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    ListNode { next: None, val }
  }

  pub fn tolist(vals: Vec<i32>) -> Option<Box<ListNode>> {
    let mut result = None;
    for i in (0..vals.len()).rev() {
      let cur = Box::new(ListNode {
        val: vals[i],
        next: result.take(),
      });
      result = Some(cur);
    }
    result
  }

  pub fn tovec(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut curr = head;
    while let Some(node) = curr.take() {
      result.push(node.val);
      curr = node.next;
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
  fn test() {
    let input = vec![1, 2, 3, 4, 5];
    let head = ListNode::tolist(input.clone());
    let output = ListNode::tovec(head);
    assert_eq!(input, output);
  }
}
