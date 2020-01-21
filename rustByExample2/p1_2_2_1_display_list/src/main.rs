use std::fmt;

#[derive(Debug)]
struct List(Vec<i32>);

impl fmt::Display for List {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let vec = &self.0;

    write!(f, "[")?;

    for (count, v) in vec.iter().enumerate() {
      if count != 0 {
        write!(f, ", ")?;
      }
      write!(f, "{}: {}", count, v)?;
    }

    write!(f, "]")
  }
}

fn main() {
  let v = List(vec![1, 2, 3]);
  println!("Display: {}", v);
  println!("Debug: {:?}", v);
  println!("Pretty Debug: {:#?}", v);
}
