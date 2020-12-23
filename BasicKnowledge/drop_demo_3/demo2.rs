struct Inner;
struct Outer(Inner);

impl Drop for Inner {
  fn drop(&mut self) {
    println!("Dropping for Inner");
  }
}

impl Drop for Outer {
  fn drop(&mut self) {
    println!("Dropping for Outer");
  }
}

fn main() {
  let _x = Outer(Inner);
}
