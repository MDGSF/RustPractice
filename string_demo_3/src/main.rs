// .retain() for string is just like .filter() for iterator.

fn main() {
  let mut my_string = String::from("Age: 20 Height, 194 Weight: 80");
  my_string.retain(|character| character.is_alphabetic() || character == ' ');
  dbg!(my_string);
}
