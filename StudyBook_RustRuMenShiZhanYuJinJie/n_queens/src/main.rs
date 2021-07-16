use n_queens::solution1;

fn main() {
  let result = solution1::Solution::solve_n_queens(4);
  println!("result = {:?}", result);
}
