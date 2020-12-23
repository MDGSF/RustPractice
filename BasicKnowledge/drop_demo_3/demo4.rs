use std::rc::Rc;

struct Foo;

impl Drop for Foo {
  fn drop(&mut self) {
    println!("dropped!");
  }
}

fn main() {
  let foo = Rc::new(Foo);
  let foo2 = Rc::clone(&foo);

  drop(foo);

  println!("drop foo");

  drop(foo2);

  println!("main end");
}
