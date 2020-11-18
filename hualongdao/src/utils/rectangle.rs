use super::*;
use std::cmp::Ordering;
use std::ops::Add;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Rect {
  left_top: Point,
  right_bottom: Point,
}

impl Rect {
  pub fn new(left_top: Point, right_bottom: Point) -> Rect {
    Rect {
      left_top,
      right_bottom,
    }
  }

  pub fn contains(&self, point: Point) -> bool {
    point.row >= self.left_top.row
      && point.row <= self.right_bottom.row
      && point.col >= self.left_top.col
      && point.col <= self.right_bottom.col
  }
}
