use std::sync::Arc;
fn main() {
  let foo = Arc::new(vec![1, 2, 3]);
  let a = foo.clone();
  let b = Arc::clone(&foo);
  let c = (*foo).clone();

  println!("foo = {:?}", foo);
  println!("a = {:?}", a);
  println!("b = {:?}", b);
  println!("c = {:?}", c);

  //let _: u8 = foo;
  //let _: u8 = a;
  //let _: u8 = b;
  //let _: u8 = c;
}
