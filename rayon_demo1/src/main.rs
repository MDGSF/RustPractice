use rayon::prelude::*;

fn sum_of_squares(input: &[i32]) -> i32 {
  input.par_iter().map(|&i| i * i).sum()
}

fn main() {
  let input = vec![1, 2, 3];
  let result = sum_of_squares(&input);
  println!("result = {}", result);
}
