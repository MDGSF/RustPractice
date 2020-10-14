fn main() {
  let numbers_together = "7431847318943194719";

  for (index, number) in numbers_together.char_indices() {
    match (index % 3, number) {
      (0..=1, number) => print!("{}", number),
      _ => print!("{}\t", number),
    }
  }
}
