use list::ListNode;

impl Solution {
  pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
    let mut slow = &mut dummy;
    let mut fast = &mut slow.clone(); //应该会clone整个链表，所以该方法不好

    // fast向后移动n+1个节点，使fast和slow之间间隔n个节点
    for _ in 1..=n + 1 {
      fast = &mut fast.as_mut().unwrap().next;
    }

    while fast.is_some() {
      fast = &mut fast.as_mut().unwrap().next;
      slow = &mut slow.as_mut().unwrap().next;
    }

    let next = &slow.as_mut().unwrap().next.as_mut().unwrap().next;
    slow.as_mut().unwrap().next = next.clone();

    dummy.unwrap().next
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    let head = ListNode::tolist(vec![1, 2, 3, 4, 5]);
    let result = Solution::remove_nth_from_end(head, 2);
    let output = ListNode::tovec(result);
    assert_eq!(output, vec![1, 2, 3, 5]);
  }
}
