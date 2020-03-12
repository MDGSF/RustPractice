use std::rc::Rc;

struct Foo;

impl Drop for Foo {
  fn drop(&mut self) {
    println!("dropped!");
  }
}

fn main() {
  let foo = Rc::new(Foo);
  let weak_foo = Rc::downgrade(&foo);
  let oterh_weak_foo = Rc::downgrade(&foo);

  drop(weak_foo);

  println!("drop weak_foo");

  drop(foo);

  println!("main end");
}
