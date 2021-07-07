use list::ListNode;

impl Solution {
  pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut fast = &head;
    let mut slow = &head;
    while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
      slow = &slow.as_ref().unwrap().next;
      fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
    }
    slow.clone()
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
    let head = ListNode::tolist(vec![1, 2, 3, 4, 5]);
    let result = Solution::middle_node(head);
    let output = ListNode::tovec(result);
    assert_eq!(output, vec![3, 4, 5]);
  }

  #[test]
  fn test_2() {
    let head = ListNode::tolist(vec![1, 2, 3, 4, 5, 6]);
    let result = Solution::middle_node(head);
    let output = ListNode::tovec(result);
    assert_eq!(output, vec![4, 5, 6]);
  }
}
