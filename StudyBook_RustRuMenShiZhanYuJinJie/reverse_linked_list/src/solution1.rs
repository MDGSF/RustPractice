use list::ListNode;

impl Solution {
  pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    let mut curr = head;
    while let Some(mut curr_node) = curr.take() {
      let next_temp = curr_node.next.take();
      curr_node.next = prev.take();
      prev = Some(curr_node);
      curr = next_temp;
    }
    prev
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    let input = vec![1, 2, 3, 4, 5];
    let head = ListNode::tolist(input.clone());
    let result = Solution::reverse_list(head);
    let output = ListNode::tovec(result);
    assert_eq!(output, vec![5, 4, 3, 2, 1]);
  }
}
