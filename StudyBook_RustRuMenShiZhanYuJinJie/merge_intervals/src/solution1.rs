impl Solution {
  pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if intervals.is_empty() {
      return intervals;
    }

    intervals.sort();
    let mut merged: Vec<Vec<i32>> = Vec::new();
    for i in 0..intervals.len() {
      let len = merged.len();
      if merged.is_empty() || merged[len - 1][1] < intervals[i][0] {
        merged.push(intervals[i].clone());
      } else {
        merged[len - 1][1] = merged[len - 1][1].max(intervals[i][1]);
      }
    }
    merged
  }
}

pub struct Solution;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(
      Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
      vec![vec![1, 6], vec![8, 10], vec![15, 18]]
    );
  }
}
