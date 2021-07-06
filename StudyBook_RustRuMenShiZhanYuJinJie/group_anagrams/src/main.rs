use group_anagrams::solution1;

fn main() {
  let result = solution1::Solution::group_anagrams(vec![
    "eat".to_string(),
    "tea".to_string(),
    "tan".to_string(),
    "ate".to_string(),
    "nat".to_string(),
    "bat".to_string(),
  ]);
  println!("{:?}", result);
}
