use std::ptr::NonNull;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<NonNull<ListNode>>,
}

impl ListNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    ListNode { next: None, val }
  }

  pub fn has_cycle(head: Option<NonNull<ListNode>>) -> bool {
    unsafe {
      let mut slow = head;
      let mut fast = head;
      while slow.is_some() && fast.is_some() && (*fast.unwrap().as_ptr()).next.is_some() {
        slow = (*slow.unwrap().as_ptr()).next;
        fast = (*fast.unwrap().as_ptr()).next;
        fast = (*fast.unwrap().as_ptr()).next;
        if slow == fast {
          return true;
        }
      }
      false
    }
  }

  pub fn detect_cycle(head: Option<NonNull<ListNode>>) -> Option<NonNull<ListNode>> {
    unsafe {
      let mut ptr1 = ListNode::get_intersection(head);
      if ptr1.is_none() {
        return None;
      }

      let mut ptr2 = head;
      while ptr1.is_some() && ptr2.is_some() {
        ptr1 = (*ptr1.unwrap().as_ptr()).next;
        ptr2 = (*ptr2.unwrap().as_ptr()).next;
        if ptr1 == ptr2 {
          return ptr1;
        }
      }
      return None;
    }
  }

  pub fn get_intersection(head: Option<NonNull<ListNode>>) -> Option<NonNull<ListNode>> {
    unsafe {
      let mut slow = head;
      let mut fast = head;
      while slow.is_some() && fast.is_some() && (*fast.unwrap().as_ptr()).next.is_some() {
        slow = (*slow.unwrap().as_ptr()).next;
        fast = (*fast.unwrap().as_ptr()).next;
        fast = (*fast.unwrap().as_ptr()).next;
        if slow == fast {
          return slow;
        }
      }
      None
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_has_cycle_1() {
    unsafe {
      let n1 = Box::new(ListNode::new(1));
      let n2 = Box::new(ListNode::new(2));
      let n3 = Box::new(ListNode::new(3));
      let n4 = Box::new(ListNode::new(4));

      let n1_ptr: NonNull<ListNode> = Box::leak(n1).into();
      let n2_ptr: NonNull<ListNode> = Box::leak(n2).into();
      let n3_ptr: NonNull<ListNode> = Box::leak(n3).into();
      let n4_ptr: NonNull<ListNode> = Box::leak(n4).into();

      (*n1_ptr.as_ptr()).next = Some(n2_ptr);
      (*n2_ptr.as_ptr()).next = Some(n3_ptr);
      (*n3_ptr.as_ptr()).next = Some(n4_ptr);

      let head = Some(n1_ptr);

      assert_eq!(ListNode::has_cycle(head), false);
    }
  }

  #[test]
  fn test_has_cycle_2() {
    unsafe {
      let n1 = Box::new(ListNode::new(1));
      let n2 = Box::new(ListNode::new(2));
      let n3 = Box::new(ListNode::new(3));
      let n4 = Box::new(ListNode::new(4));

      let n1_ptr: NonNull<ListNode> = Box::leak(n1).into();
      let n2_ptr: NonNull<ListNode> = Box::leak(n2).into();
      let n3_ptr: NonNull<ListNode> = Box::leak(n3).into();
      let n4_ptr: NonNull<ListNode> = Box::leak(n4).into();

      (*n1_ptr.as_ptr()).next = Some(n2_ptr);
      (*n2_ptr.as_ptr()).next = Some(n3_ptr);
      (*n3_ptr.as_ptr()).next = Some(n4_ptr);

      (*n4_ptr.as_ptr()).next = Some(n2_ptr);

      let head = Some(n1_ptr);

      assert_eq!(ListNode::has_cycle(head), true);
    }
  }

  #[test]
  fn test_detect_cycle_1() {
    unsafe {
      let n1 = Box::new(ListNode::new(1));
      let n2 = Box::new(ListNode::new(2));
      let n3 = Box::new(ListNode::new(3));
      let n4 = Box::new(ListNode::new(4));

      let n1_ptr: NonNull<ListNode> = Box::leak(n1).into();
      let n2_ptr: NonNull<ListNode> = Box::leak(n2).into();
      let n3_ptr: NonNull<ListNode> = Box::leak(n3).into();
      let n4_ptr: NonNull<ListNode> = Box::leak(n4).into();

      (*n1_ptr.as_ptr()).next = Some(n2_ptr);
      (*n2_ptr.as_ptr()).next = Some(n3_ptr);
      (*n3_ptr.as_ptr()).next = Some(n4_ptr);

      (*n4_ptr.as_ptr()).next = Some(n2_ptr);

      let head = Some(n1_ptr);

      let entry_node = ListNode::detect_cycle(head);
      assert_eq!(entry_node.is_some(), true);
      assert_eq!((*entry_node.unwrap().as_ptr()).val, 2);
    }
  }
}
