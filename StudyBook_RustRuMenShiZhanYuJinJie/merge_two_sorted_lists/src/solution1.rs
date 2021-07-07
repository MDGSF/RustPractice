use list::ListNode;

impl Solution {
  pub fn merge_two_lists(
    listnode1: Option<Box<ListNode>>,
    listnode2: Option<Box<ListNode>>,
  ) -> Option<Box<ListNode>> {
    match (listnode1, listnode2) {
      (Some(node1), None) => Some(node1),
      (None, Some(node2)) => Some(node2),
      (Some(node1), Some(node2)) => {
        let (mut small, big) = if node1.val < node2.val {
          (node1, node2)
        } else {
          (node2, node1)
        };
        let small_tail = small.next.take();
        small.next = Solution::merge_two_lists(small_tail, Some(big));
        Some(small)
      }
      _ => None,
    }
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    let listnode1 = ListNode::tolist(vec![1, 2, 4]);
    let listnode2 = ListNode::tolist(vec![1, 3, 4]);
    let result = Solution::merge_two_lists(listnode1, listnode2);
    let output = ListNode::tovec(result);
    assert_eq!(output, vec![1, 1, 2, 3, 4, 4]);
  }
}
