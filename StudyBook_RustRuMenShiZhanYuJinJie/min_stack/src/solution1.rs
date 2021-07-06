pub struct MinStack {
  stack: Vec<i32>,
  min_stack: Vec<i32>,
}

impl MinStack {
  pub fn new() -> Self {
    MinStack {
      stack: Vec::new(),
      min_stack: Vec::new(),
    }
  }

  pub fn push(&mut self, x: i32) {
    self.stack.push(x);
    if self.min_stack.is_empty() || x < *self.min_stack.last().unwrap() {
      self.min_stack.push(x);
    }
  }

  pub fn pop(&mut self) {
    if self.stack.is_empty() {
      return;
    }
    if self.stack.pop().unwrap() == *self.min_stack.last().unwrap() {
      self.min_stack.pop();
    }
  }

  pub fn top(&self) -> i32 {
    *self.stack.last().unwrap()
  }

  pub fn get_min(&self) -> i32 {
    *self.min_stack.last().unwrap()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    let mut s = MinStack::new();
    s.push(-2);
    s.push(0);
    s.push(-3);
    assert_eq!(s.get_min(), -3);
    s.pop();
    assert_eq!(s.top(), 0);
    assert_eq!(s.get_min(), -2);
  }
}
