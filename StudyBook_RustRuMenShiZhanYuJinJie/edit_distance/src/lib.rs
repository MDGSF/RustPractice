impl Solution {
  pub fn min_distance(word1: String, word2: String) -> i32 {
    let word1_chars: Vec<char> = word1.chars().collect();
    let word2_chars: Vec<char> = word2.chars().collect();

    let rows = word1.len() + 1;
    let cols = word2.len() + 1;

    let mut dp = vec![vec![0; cols]; rows];

    for col in 1..cols {
      dp[0][col] = dp[0][col - 1] + 1;
    }

    for row in 1..rows {
      dp[row][0] = dp[row - 1][0] + 1;
    }

    for row in 1..rows {
      for col in 1..cols {
        if word1_chars[row - 1] == word2_chars[col - 1] {
          dp[row][col] = dp[row - 1][col - 1];
        } else {
          dp[row][col] = dp[row - 1][col - 1]
            .min(dp[row - 1][col])
            .min(dp[row][col - 1])
            + 1;
        }
      }
    }

    dp[rows - 1][cols - 1]
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(
      Solution::min_distance("horse".to_string(), "ros".to_string()),
      3
    );

    assert_eq!(
      Solution::min_distance("intention".to_string(), "execution".to_string()),
      5
    );
  }
}
