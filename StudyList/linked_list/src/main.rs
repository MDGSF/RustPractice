use linked_list::list::LinkedList;

fn main() {
  let mut list1 = LinkedList::new();
  list1.push_back('a');
  list1.push_back('b');
  list1.push_back('c');
  let mut iter = list1.iter();
  println!("{:?}", iter.next());
  println!("{:?}", iter.next());
  println!("{:?}", iter.next());
  println!("{:?}", iter.next());
}
