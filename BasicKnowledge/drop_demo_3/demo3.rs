struct PrintOnDrop(&'static str);

impl Drop for PrintOnDrop {
  fn drop(&mut self) {
    println!("{}", self.0);
  }
}

fn main() {
  let _first = PrintOnDrop("Declared first!");
  let _second = PrintOnDrop("Declared second!");
}
