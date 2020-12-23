fn main() {
  let number_vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];

  for chunk in number_vec.chunks(3) {
    println!("{:?}", chunk);
  }
  println!();

  for window in number_vec.windows(3) {
    println!("{:?}", window);
  }
}
