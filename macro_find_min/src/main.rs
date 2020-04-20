macro_rules! find_min {
  ($x:expr) => ($x);
  ($x:expr, $($y:expr),+) => (
    std::cmp::min($x, find_min!($($y),+))
  )
}

fn main() {
  println!("{}", find_min!(1));
  println!("{}", find_min!(2, 3));
  println!("{}", find_min!(4, 5, 6));
}
