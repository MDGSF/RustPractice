#[cfg(feature = "test1")]
fn test1() {
  println!("I'm test1")
}

#[cfg(feature = "test2")]
fn test2() {
  println!("I'm test2")
}

#[cfg(feature = "test3")]
fn test3() {
  println!("I'm test3")
}

#[cfg(feature = "test4")]
fn test4() {
  println!("I'm test4")
}

fn main() {
  println!("Hello, world!");

  test1();
  test2();

  test3();

  test4();
}
