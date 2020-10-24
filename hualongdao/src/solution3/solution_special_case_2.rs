use super::*;
use std::cmp::Ordering;

impl Solution {
  // 只剩下 3 * 3 大小的空间了，改用暴力搜索
  pub(super) fn special_2_condition(&self) -> bool {
    let left_width = self.end_col - self.start_col + 1;
    let left_height = self.end_row - self.start_row + 1;
    if left_width <= 3 && left_height <= 3 {
      return true;
    }
    false
  }

  pub(super) fn specail_2_process(&mut self) {
    info!("{}", self);
    panic!("show 3 * 3");
  }
}

#[derive(Debug, Clone, Eq)]
struct AStarContext {
  manhattan_distance: usize,
  board_str: String,
  path: Vec<String>,
}

impl Ord for AStarContext {
  fn cmp(&self, other: &Self) -> Ordering {
    other.manhattan_distance.cmp(&self.manhattan_distance)
  }
}

impl PartialOrd for AStarContext {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl PartialEq for AStarContext {
  fn eq(&self, other: &Self) -> bool {
    self.manhattan_distance == other.manhattan_distance
  }
}
