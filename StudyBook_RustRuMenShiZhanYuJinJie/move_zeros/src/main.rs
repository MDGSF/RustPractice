use move_zeros::solution1;

fn main() {
  let mut nums = vec![0, 1, 0, 3, 12];
  solution1::Solution::move_zeros(&mut nums);
  println!("{:?}", nums);
}
