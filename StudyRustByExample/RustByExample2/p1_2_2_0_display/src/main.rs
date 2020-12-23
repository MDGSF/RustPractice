use std::fmt;

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({}, {})", self.0, self.1)
  }
}

#[derive(Debug)]
struct Point2D {
  x: f64,
  y: f64,
}

impl fmt::Display for Point2D {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "x: {}, y: {}", self.x, self.y)
  }
}

#[derive(Debug)]
struct Complex {
  real: f64,
  imag: f64,
}

impl fmt::Display for Complex {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{} + {}i", self.real, self.imag)
  }
}

fn main() {
  let minmax = MinMax(0, 28);
  println!("Display: {}", minmax);
  println!("Debug: {:?}", minmax);
  println!("Pretty Debug: {:#?}", minmax);

  let big_range = MinMax(-300, 300);
  let small_range = MinMax(-3, 3);
  println!(
    "The big range is {big} and the small is {small}",
    small = small_range,
    big = big_range
  );

  let point = Point2D { x: 3.3, y: 7.2 };
  println!("Display: {}", point);
  println!("Debug: {:?}", point);
  println!("Pretty Debug: {:#?}", point);

  let complex = Complex {
    real: 1f64,
    imag: 8.6,
  };
  println!("Display: {}", complex);
  println!("Debug: {:?}", complex);
  println!("Pretty Debug: {:#?}", complex);
}
