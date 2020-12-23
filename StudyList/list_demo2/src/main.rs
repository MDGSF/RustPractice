use std::cell::RefCell;
use std::collections::LinkedList;
use std::sync::{Arc, Mutex, Weak};

struct List {
  len: i32,
  tail: Option<Arc<RefCell<Node>>>,
  head: Option<Arc<RefCell<Node>>>,
}

struct Node {
  val: i32,
  prev: Option<Arc<RefCell<Node>>>,
  next: Option<Arc<RefCell<Node>>>,
}

impl List {
  fn new() -> List {
    let list = List {
      len: 0,
      tail: None,
      head: None,
    };
    list
  }

  fn len(&self) -> i32 {
    self.len
  }

  fn push_back(&mut self, val: i32) {
    self.push_back_node(Arc::new(RefCell::new(Node::new(val))))
  }

  fn push_back_node(&mut self, node: Arc<RefCell<Node>>) {
    if self.len == 0 {
      self.head = Some(node.clone());
      self.tail = Some(node.clone());
    } else {
      node.borrow_mut().prev = Some(self.tail.unwrap().clone());
    }
    self.len += 1;
  }
}

impl Node {
  fn new(val: i32) -> Node {
    Node {
      val,
      prev: None,
      next: None,
    }
  }
}

fn main() {
  println!("Hello, world!");

  // let list: LinkedList<u32> = LinkedList::new();

  let mut list = List::new();
  let listlen = list.len();
  println!("listlen = {}", listlen);

  list.push_back(3);
  let listlen = list.len();
  println!("listlen = {}", listlen);
}
