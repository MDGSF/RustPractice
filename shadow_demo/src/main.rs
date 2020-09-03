struct HasDrop;

impl Drop for HasDrop {
  fn drop(&mut self) {
    println!("Dropping HasDrop!");
  }
}

// 变量被遮蔽的时候，内存并没有被释放
fn main() {
  println!("Hello, world!");
  let _a = HasDrop {};
  println!("111");
  let _a = 123;
  println!("222");
}
