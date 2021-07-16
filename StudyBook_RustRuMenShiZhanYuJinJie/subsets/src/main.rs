use subsets::solution1;

fn main() {
  let nums = vec![1, 2, 3];
  let result = solution1::Solution::subsets(nums);
  println!("result = {:?}", result);
}
