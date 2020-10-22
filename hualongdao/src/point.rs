use super::*;
use std::ops::Add;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct UPoint {
  pub row: usize,
  pub col: usize,
}

impl UPoint {
  pub fn newi(point: IPoint) -> UPoint {
    UPoint {
      row: point.row as usize,
      col: point.col as usize,
    }
  }
}

impl Add<Direction> for UPoint {
  type Output = IPoint;

  fn add(self, other: Direction) -> IPoint {
    IPoint {
      row: self.row as isize + other.row,
      col: self.col as isize + other.col,
    }
  }
}

impl Add<&Direction> for UPoint {
  type Output = IPoint;

  fn add(self, other: &Direction) -> IPoint {
    IPoint {
      row: self.row as isize + other.row,
      col: self.col as isize + other.col,
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
