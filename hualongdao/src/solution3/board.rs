use super::*;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Board(Vec<Vec<usize>>);

impl Board {
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
  pub fn swap_two_points(&mut self, point1: Point, point2: Point) {
    let temp = self[point1];
    self[point1] = self[point2];
    self[point2] = temp;
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
