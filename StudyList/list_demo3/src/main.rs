use std::collections::LinkedList;

fn main() {
  let mut list: LinkedList<Vec<u8>> = LinkedList::new();
  let e1 = vec![1, 2, 3];
  let e2 = vec![4, 5, 6];
  list.push_back(e1);
  list.push_back(e2);
  for elem in list.iter() {
    println!("{:?}", elem);
  }
}

fn test1() {
  let mut list: LinkedList<u32> = LinkedList::new();
  list.push_back(1);
  list.push_back(2);

  for elem in list.iter() {
    println!("{}", elem);
  }
}
