struct Example {
  a: i32,
}

impl Drop for Example {
  fn drop(&mut self) {
    println!("Dropping the instance of Example with data: {}", self.a);
  }
}

fn main() {
  println!("Hello, world!");
  let _a1 = Example { a: 10 };
  let _a2 = Example { a: 20 };
  println!("Instances of Example type are created");
}
