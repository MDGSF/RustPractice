use std::cell::Ref;
use std::cell::RefCell;
use std::cell::RefMut;

fn main() {
  let v: RefCell<Vec<i32>> = RefCell::new(vec![1, 2, 3, 4]);

  let v_borrow: Ref<Vec<i32>> = v.borrow();
  println!("{:?}", v_borrow);

  let mut v_borrow_mut_2: RefMut<Vec<i32>> = v.borrow_mut();
  v_borrow_mut_2.push(6);
  println!("{:?}", v_borrow_mut_2);
}
