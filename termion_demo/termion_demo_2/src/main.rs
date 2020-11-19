extern crate termion;

fn main() {
  println!(
    "{}{}Stuff",
    termion::clear::All,
    termion::cursor::Goto(5, 3)
  );
}
