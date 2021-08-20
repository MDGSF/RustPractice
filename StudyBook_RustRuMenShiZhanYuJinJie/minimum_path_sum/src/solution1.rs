impl Solution {
  pub fn min_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
    let rows = matrix.len();
    if rows == 0 {
      return 0;
    }
    let cols = matrix[0].len();
    if cols == 0 {
      return 0;
    }

    let mut dp = vec![vec![0; cols]; rows];

    let mut sum = 0;
    for row in 0..rows {
      sum += matrix[row][0];
      dp[row][0] = sum;
    }

    sum = 0;
    for col in 0..cols {
      sum += matrix[0][col];
      dp[0][col] = sum;
    }

    for row in 1..rows {
      for col in 1..cols {
        dp[row][col] = matrix[row][col] + dp[row - 1][col].min(dp[row][col - 1]);
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
      Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]),
      7
    );
  }
}
