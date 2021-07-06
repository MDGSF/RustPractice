impl Solution {
  pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    let mut carry = 1;
    for i in (0..digits.len()).rev() {
      digits[i] += carry;
      carry = digits[i] / 10;
      digits[i] %= 10;
    }
    if carry > 0 {
      digits.insert(0, carry);
    }
    digits
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    assert_eq!(Solution::plus_one(vec![9, 9, 9]), vec![1, 0, 0, 0]);
  }
}
