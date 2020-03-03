#[derive(Debug)]
struct Queue<T> {
  qdata: Vec<T>,
}

impl<T> Queue<T> {
  fn new() -> Self {
    Queue { qdata: Vec::new() }
  }

  fn push(&mut self, item: T) {
    self.qdata.push(item);
  }

  fn pop(&mut self) -> Option<T> {
    if self.qdata.len() == 0 {
      return None;
    }
    Some(self.qdata.remove(0))
  }
}

fn main() {
  let mut q = Queue::new();
  q.push(1);
  q.push(2);
  println!("{:?}", q);
  q.pop();
  println!("{:?}", q);
  q.pop();
  q.pop();
}
