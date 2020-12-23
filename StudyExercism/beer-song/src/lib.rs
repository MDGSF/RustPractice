pub fn verse(n: u32) -> String {
  if n == 0 {
    String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n")
  } else if n == 1 {
    String::from("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n")
  } else if n == 2 {
    format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottle of beer on the wall.\n", n, n, n - 1)
  } else {
    format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n, n, n - 1)
  }
}

pub fn sing(start: u32, end: u32) -> String {
  let mut result = String::new();
  for n in (end..=start).rev() {
    result.push_str(&verse(n));
    result.push_str("\n");
  }
  result.pop();
  result
}
