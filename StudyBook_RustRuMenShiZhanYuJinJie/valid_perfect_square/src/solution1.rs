impl Solution {
  pub fn is_perfect_square(num: i32) -> bool {
    if num == 0 || num == 1 {
      return true;
    }

    let mut left = 2;
    let mut right = num / 2;
    while left <= right {
      let mid = left + (right - left) / 2;
      let guess_suqared = mid as i64 * mid as i64;
      if guess_suqared == num as i64 {
        return true;
      } else if guess_suqared > num as i64 {
        right = mid - 1;
      } else {
        left = mid + 1;
      }
    }

    return false;
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(Solution::is_perfect_square(16), true);
    assert_eq!(Solution::is_perfect_square(14), false);
  }
}
