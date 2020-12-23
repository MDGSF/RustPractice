use std::rc::Rc;

fn main() {
  println!("Hello, world!");
  let five = Rc::new(5);
  let five2 = five.clone();
  println!("five = {}", five);
  println!("five2 = {}", five2);
}
