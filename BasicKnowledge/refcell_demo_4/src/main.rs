use std::cell::RefCell;
use std::cell::RefMut;

fn main() {
  let v: RefCell<Vec<i32>> = RefCell::new(vec![1, 2, 3, 4]);

  let mut v_borrow_mut_1: RefMut<Vec<i32>> = v.borrow_mut();
  v_borrow_mut_1.push(5);
  println!("{:?}", v_borrow_mut_1);

  let mut v_borrow_mut_2: RefMut<Vec<i32>> = v.borrow_mut();
  v_borrow_mut_2.push(6);
  println!("{:?}", v_borrow_mut_2);
}
