use std::ops::RangeFull;
fn main() {
  let (.., x, y) = (0, 1, ..);
  println!("{}", b"066"[y][x]);
}
