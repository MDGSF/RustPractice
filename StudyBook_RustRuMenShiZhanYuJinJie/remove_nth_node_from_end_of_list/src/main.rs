use list::ListNode;
use remove_nth_node_from_end_of_list::solution1;

fn _test_clone() {
  let mut head = ListNode::tolist(vec![1, 2, 3, 4, 5]);
  let p1 = &mut head;
  let p2 = &mut p1.clone();
  println!("p1 = {:?}", p1);
  println!("p2 = {:?}", p2);

  p1.as_mut().unwrap().val = 10;
  p2.as_mut().unwrap().val = 20;
  println!("p1 = {:?}", p1);
  println!("p2 = {:?}", p2);
}

fn main() {
  let head = ListNode::tolist(vec![1, 2, 3, 4, 5]);
  let result = solution1::Solution::remove_nth_from_end(head, 2);
  println!("{:?}", result);
}
