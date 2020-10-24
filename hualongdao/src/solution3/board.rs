use super::*;
use std::fmt;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Board(Vec<Vec<usize>>);

impl Board {
  pub fn new(board: Vec<Vec<usize>>) -> Board {
    Board(board)
  }

  /// 在 board 上遍历查找指定的 num 数字，返回其下标。
  pub fn find_num(&self, num: usize) -> Option<Point> {
    let board = &self.0;
    for row in 0..board.len() {
      for col in 0..board[row].len() {
        if board[row][col] == num {
          return Some(Point { row, col });
        }
      }
    }
    None
  }

  /// 交换 board 上面的 point1 和 point2 两个点的值。
  pub fn swap_points(&mut self, point1: Point, point2: Point) {
    let temp = self[point1];
    self[point1] = self[point2];
    self[point2] = temp;
  }

  /// 计算 num 数字应该在正方形上的位置。
  ///
  /// # Example
  /// num = 4, size = 3, output = Point { 1, 0 }
  /// 1 2 3
  /// 4 5 6
  /// 7 8 9
  pub fn number_to_point(&self, num: usize) -> Point {
    let size = self.0.len();
    let row = (num - 1) / size;
    let col = (num - 1) % size;
    Point { row, col }
  }

  /// 计算 point 位置应该放的数字 num
  pub fn point_to_number(&self, point: Point) -> usize {
    let size = self.0.len();
    point.row * size + point.col + 1
  }
}

impl Index<Point> for Board {
  type Output = usize;

  fn index(&self, point: Point) -> &Self::Output {
    &self.0[point.row][point.col]
  }
}

impl IndexMut<Point> for Board {
  fn index_mut(&mut self, point: Point) -> &mut Self::Output {
    &mut self.0[point.row][point.col]
  }
}

impl fmt::Display for Board {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut result = String::new();
    let board = &self.0;
    for row in 0..board.len() {
      result.push_str("[ ");
      for col in 0..board[row].len() {
        result.push_str(&format!("{:>2} ", board[row][col]));
      }
      result.push_str("]\n");
    }
    write!(f, "{}", result)
  }
}
