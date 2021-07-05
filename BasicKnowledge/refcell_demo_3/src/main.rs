use std::cell::Ref;
use std::cell::RefCell;

fn main() {
  let v: RefCell<Vec<i32>> = RefCell::new(vec![1, 2, 3, 4]);

  let v_borrow_1: Ref<Vec<i32>> = v.borrow();
  println!("{:?}", v_borrow_1);

  let v_borrow_2: Ref<Vec<i32>> = v.borrow();
  println!("{:?}", v_borrow_2);
}
