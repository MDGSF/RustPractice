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

    result.push_str(&format!("{}", self.board));

    result.push_str(&format!("fixed: {}, {:?}\n", self.fixed, self.fixed_point));

    let mut fixed_points: Vec<_> = self.fixed_points.iter().collect();
    fixed_points.sort();
    result.push_str(&format!("fixed_points: {:?}\n", fixed_points));
    result.push_str(&format!("zero_point: {:?}\n", self.zero_point));
    result.push_str(&format!(
      "start_row: {}, start_col: {}\n",
      self.start_row, self.start_col
    ));
    result.push_str(&format!(
      "end_row: {}, end_col: {}\n",
      self.end_row, self.end_col
    ));
    result.push_str(&format!("result: {}\n", self.result.join("")));
    write!(f, "{}", result)
  }
}
