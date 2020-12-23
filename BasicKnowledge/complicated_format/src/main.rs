fn main() {
  let title = "TODAY'S NEWS";
  // no variable name, pad with -, put in center, 30 characters long
  println!("{:-^30}", title);

  let bar = "|";
  // no variable name, pad with space,
  // 15 characters each, one to the left, one to the righ
  println!("{: <15}{: >15}", bar, bar);

  let a = "SEOUL";
  let b = "TOKYO";
  // variable names city1 and city2, pad with -
  // one to the left, one to the right
  println!("{city1:-<15}{city2:->15}", city1 = a, city2 = b);
}
