use super::*;
use std::cmp::Ordering;
use std::ops::Add;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Point {
  pub row: usize,
  pub col: usize,
}

impl Point {
  pub fn new(row: usize, col: usize) -> Point {
    Point { row, col }
  }
}

impl Add<Direction> for Point {
  type Output = IPoint;

  fn add(self, other: Direction) -> IPoint {
    IPoint {
      row: self.row as isize + other.row,
      col: self.col as isize + other.col,
    }
  }
}

impl Add<&Direction> for Point {
  type Output = IPoint;

  fn add(self, other: &Direction) -> IPoint {
    IPoint {
      row: self.row as isize + other.row,
      col: self.col as isize + other.col,
    }
  }
}

impl Ord for Point {
  fn cmp(&self, other: &Self) -> Ordering {
    match self.row.cmp(&other.row) {
      Ordering::Equal => self.col.cmp(&other.col),
      Ordering::Less => Ordering::Less,
      Ordering::Greater => Ordering::Greater,
    }
  }
}

impl PartialOrd for Point {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl From<IPoint> for Point {
  fn from(point: IPoint) -> Self {
    Point {
      row: point.row as usize,
      col: point.col as usize,
    }
  }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct IPoint {
  pub row: isize,
  pub col: isize,
}

impl Add<Direction> for IPoint {
  type Output = Self;

  fn add(self, other: Direction) -> Self {
    Self {
      row: self.row + other.row,
      col: self.col + other.col,
    }
  }
}

impl Add<&Direction> for IPoint {
  type Output = Self;

  fn add(self, other: &Direction) -> Self {
    Self {
      row: self.row + other.row,
      col: self.col + other.col,
    }
  }
}

/// 计算 point 相对于原点 origin_point 的相对位置
pub fn calc_two_point_relative_position(origin_point: Point, point: Point) -> Point {
  Point {
    row: point.row - origin_point.row,
    col: point.col - origin_point.col,
  }
}

/// 计算两个点之间的曼哈顿距离
pub fn calc_two_point_manhattan_distance(p1: Point, p2: Point) -> usize {
  (p1.row as isize - p2.row as isize).abs() as usize
    + (p1.col as isize - p2.col as isize).abs() as usize
}
