use super::*;
use std::fmt;

impl fmt::Display for Solution {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut result = String::new();
    result.push_str(&format!("stage: {}\n", self.stage));
    result.push_str(&format!(
      "size: {} * {} = {}\n",
      self.size, self.size, self.max_number,
    ));
    for row in 0..self.board.len() {
      result.push_str("[ ");
      for col in 0..self.board[row].len() {
        result.push_str(&format!("{:>2} ", self.board[row][col]));
      }
      result.push_str("]\n");
    }

    result.push_str(&format!("fixed: {}, {:?}\n", self.fixed, self.fixed_point));
    result.push_str(&format!("fixed_points: {:?}\n", self.fixed_points));
    result.push_str(&format!("zero_point: {:?}\n", self.zero_point));
    result.push_str(&format!("result: {}\n", self.result.join("")));
    write!(f, "{}", result)
  }
}
