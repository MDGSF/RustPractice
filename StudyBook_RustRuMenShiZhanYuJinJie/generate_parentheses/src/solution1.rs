impl Solution {
  pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut vec = Vec::new();
    Solution::recursion(&mut vec, 0, 0, n, String::from(""));
    vec
  }

  fn recursion(vec: &mut Vec<String>, left: i32, right: i32, n: i32, s: String) {
    if left == n && right == n {
      vec.push(s.clone());
      return;
    }

    if left < n {
      Solution::recursion(vec, left + 1, right, n, format!("{}{}", &s, "("));
    }
    if right < left {
      Solution::recursion(vec, left, right + 1, n, format!("{}{}", &s, ")"));
    }
  }
}

pub struct Solution;
