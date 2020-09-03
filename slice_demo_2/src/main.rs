fn main() {
  let array_of_ten = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

  let three_to_five = &array_of_ten[2..5];
  let start_at_two = &array_of_ten[1..];
  let end_at_five = &array_of_ten[..5];

  println!("array_of_ten: {:?}", array_of_ten);
  println!("three_to_five: {:?}", three_to_five);
  println!("start_at_two: {:?}", start_at_two);
  println!("end_at_five: {:?}", end_at_five);
}
