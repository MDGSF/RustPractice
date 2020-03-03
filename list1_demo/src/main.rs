use List::*;

enum List {
  Cons(u32, Box<List>),
  Nil,
}

impl List {
  fn new() -> List {
    Nil
  }

  fn prepend(self, elem: u32) -> List {
    Cons(elem, Box::new(self))
  }

  fn len(&self) -> u32 {
    // &self 是 self: &List 的缩写
    // self 的类型是 &List, *self 的类型是 List
    // 匹配一个类型 T 好过匹配一个引用 &T
    match *self {
      Cons(_, ref tail) => 1 + tail.len(),
      Nil => 0,
    }
  }

  fn stringify(&self) -> String {
    match *self {
      Cons(head, ref tail) => format!("{}, {}", head, tail.stringify()),
      Nil => format!("Nil"),
    }
  }
}

fn main() {
  let mut list = List::new();

  list = list.prepend(1);
  list = list.prepend(2);
  list = list.prepend(3);

  println!("linked list len = {}", list.len());
  println!("{}", list.stringify());
}
