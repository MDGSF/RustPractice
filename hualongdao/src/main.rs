#[macro_use]
extern crate lazy_static;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;
use std::ops::Add;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct InputContext {
  board: Vec<Vec<usize>>,
  fixed: usize,
  size: usize,
  stage: usize,
}

#[derive(Debug, Clone, Eq)]
struct BFSContext {
  position: UPoint,
  manhattan_distance: usize,
  path: Vec<UPoint>,
}

impl Ord for BFSContext {
  fn cmp(&self, other: &Self) -> Ordering {
    other.manhattan_distance.cmp(&self.manhattan_distance)
  }
}

impl PartialOrd for BFSContext {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl PartialEq for BFSContext {
  fn eq(&self, other: &Self) -> bool {
    self.manhattan_distance == other.manhattan_distance
  }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct UPoint {
  row: usize,
  col: usize,
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
struct IPoint {
  row: isize,
  col: isize,
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

#[derive(Debug, Clone)]
struct Direction {
  row: isize,
  col: isize,
  name: &'static str,
}

lazy_static! {
  static ref DIRECTIONS: Vec<Direction> = {
    let mut m = Vec::new();
    m.push(Direction {
      row: 0,
      col: -1,
      name: "L",
    });
    m.push(Direction {
      row: 0,
      col: 1,
      name: "R",
    });
    m.push(Direction {
      row: -1,
      col: 0,
      name: "U",
    });
    m.push(Direction {
      row: 1,
      col: 0,
      name: "D",
    });
    m
  };
}

fn swap_node(
  board: &mut Vec<Vec<usize>>,
  old_row: usize,
  old_col: usize,
  new_row: usize,
  new_col: usize,
) {
  let temp = board[old_row][old_col];
  board[old_row][old_col] = board[new_row][new_col];
  board[new_row][new_col] = temp;
}

fn find_zero_point(board: &Vec<Vec<usize>>) -> UPoint {
  for row in 0..board.len() {
    for col in 0..board[row].len() {
      if board[row][col] == 0 {
        return UPoint { row, col };
      }
    }
  }
  UPoint { row: 0, col: 0 }
}

fn calc_two_point_manhattan_distance(p1: UPoint, p2: UPoint) -> usize {
  (p1.row as isize - p2.row as isize).abs() as usize
    + (p1.col as isize - p2.col as isize).abs() as usize
}

fn number_to_point(num: usize, size: usize) -> UPoint {
  let row = (num - 1) / size;
  let col = (num - 1) % size;
  UPoint { row, col }
}

fn find_num_point(board: &Vec<Vec<usize>>, num: usize) -> UPoint {
  for row in 0..board.len() {
    for col in 0..board[row].len() {
      if board[row][col] == num {
        return UPoint { row, col };
      }
    }
  }
  UPoint { row: 0, col: 0 }
}

struct Solution {
  board: Vec<Vec<usize>>,        // board 是个正方形
  fixed: usize,                  // 固定点的数字
  size: usize,                   // board 的边长
  stage: usize,                  // 第几关
  fixed_point: UPoint,           // fixed 的行列位置
  max_number: usize,             // 最大数字 size * size
  fixed_points: HashSet<UPoint>, // 当前不能被移动的点
  zero_point: UPoint,            // 空格的位置
  result: Vec<String>,           // 保存最后的结果，移动命令，L R U D
}

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
        result.push_str(&format!("{} ", self.board[row][col]));
      }
      result.push_str("]\n");
    }

    result.push_str(&format!("fixed: {}, {:?}\n", self.fixed, self.fixed_point));
    result.push_str(&format!("fixed_points: {:?}\n", self.fixed_points));
    result.push_str(&format!("zero_point: {:?}\n", self.zero_point));
    result.push_str(&format!("result: {:?}\n", self.result));
    write!(f, "{}", result)
  }
}

impl Solution {
  pub fn new(input_context: &InputContext) -> Solution {
    let fixed_point = number_to_point(input_context.fixed, input_context.size);
    let max_number = input_context.size * input_context.size;
    let mut fixed_points: HashSet<UPoint> = HashSet::new();
    fixed_points.insert(fixed_point);
    let zero_point = find_num_point(&input_context.board, 0);
    Solution {
      board: input_context.board.clone(),
      fixed: input_context.fixed,
      size: input_context.size,
      stage: input_context.stage,
      fixed_point,
      max_number,
      fixed_points,
      zero_point,
      result: Vec::new(),
    }
  }

  fn process(&mut self) {
    self.process_no_fix();
  }

  fn process_no_fix(&mut self) {
    for num in 1..self.max_number {
      self.move_number(num);

      let num_point = number_to_point(num, self.size);
      self.fixed_points.insert(num_point);
    }
  }

  fn move_number(&mut self, num: usize) {
    let zero = find_zero_point(&self.board);

    println!("\n----------------");
    println!("move start num = {}", num);
    println!("{}", self);

    let num_point = number_to_point(num, self.size);

    if num % self.size == 0 {
      self.move_row_last_number(num);
    } else if num_point.row == self.size - 1 {
      self.move_col_last_number(num);
    } else {
      self.move_basic_number(num);
    }
  }

  fn move_row_last_number(&mut self, num: usize) {
    let src_point = find_num_point(&self.board, num);
    let mut dst_point = number_to_point(num, self.size);
    if src_point == dst_point {
      println!("num = {}, no need to move", num);
      return;
    }
    dst_point.row += 1;
    self.move_number_to_dst(num, src_point, dst_point);

    // TODO
  }

  fn move_col_last_number(&mut self, num: usize) {
    let src_point = find_num_point(&self.board, num);
    let mut dst_point = number_to_point(num, self.size);
    if src_point == dst_point {
      println!("num = {}, no need to move", num);
      return;
    }
    dst_point.col += 1;
    self.move_number_to_dst(num, src_point, dst_point);

    // TODO
  }

  fn move_basic_number(&mut self, num: usize) {
    let src_point = find_num_point(&self.board, num);
    let dst_point = number_to_point(num, self.size);
    if src_point == dst_point {
      println!("num = {}, no need to move", num);
      return;
    }
    self.move_number_to_dst(num, src_point, dst_point);
  }

  fn move_number_to_dst(&mut self, num: usize, src_point: UPoint, dst_point: UPoint) {
    println!("move {} from {:?} to {:?}", num, src_point, dst_point);

    let num_paths = self.find_num_path_bfs(num, src_point, dst_point);
    if num_paths.is_none() {
      println!("{}", self);
      panic!("find special case");
    }
    let num_paths = num_paths.unwrap();
    println!("num_paths = {:?}", num_paths);

    let mut num_point = src_point;
    for path_point in num_paths {
      self.fixed_points.insert(num_point);

      // 先把 0 移动到要移动的数字前面
      let zero_paths = self.find_num_path_bfs(0, self.zero_point, path_point);
      if zero_paths.is_none() {
        println!("{}", self);
        panic!("find special case");
      }
      let zero_paths = zero_paths.unwrap();

      self.fixed_points.remove(&num_point);

      self.move_zero_with_paths(zero_paths);

      // 把数字向前移动一步
      self.swap_with_zero(num_point);
      num_point = path_point;

      println!("one step, {}", self);
    }
  }

  fn move_zero_with_paths(&mut self, zero_paths: Vec<UPoint>) {
    for &path_point in zero_paths.iter() {
      self.swap_with_zero(path_point);
    }
  }

  fn find_num_path_bfs(
    &mut self,
    num: usize,
    src_point: UPoint,
    dst_point: UPoint,
  ) -> Option<Vec<UPoint>> {
    let context = BFSContext {
      position: src_point,
      manhattan_distance: calc_two_point_manhattan_distance(src_point, dst_point),
      path: vec![],
    };

    let mut s = HashSet::new();
    s.insert(src_point);

    let mut heap = BinaryHeap::new();
    heap.push(context);

    while !heap.is_empty() {
      let context = heap.pop().unwrap();
      let point = context.position;

      for direction in DIRECTIONS.iter() {
        let new_ipoint = point + direction;
        if !self.is_valid_ipoint(&new_ipoint) {
          continue;
        }

        let new_upoint = UPoint::newi(new_ipoint);

        if new_upoint == dst_point {
          let mut new_path = context.path.clone();
          new_path.push(new_upoint);
          return Some(new_path);
        }

        if !s.contains(&new_upoint) && !self.is_fixed_upoint(&new_upoint) {
          let mut new_path = context.path.clone();
          new_path.push(new_upoint);
          let new_context = BFSContext {
            position: new_upoint,
            manhattan_distance: calc_two_point_manhattan_distance(new_upoint, dst_point),
            path: new_path,
          };
          heap.push(new_context);
        }
      }
    }

    None
  }

  fn is_valid_ipoint(&self, point: &IPoint) -> bool {
    let size = self.size as isize;
    point.row >= 0 && point.row < size && point.col >= 0 && point.col < size
  }

  fn is_fixed_upoint(&self, point: &UPoint) -> bool {
    self.fixed_points.contains(point)
  }

  fn swap_node(&mut self, old_point: UPoint, new_point: UPoint) {
    let temp = self.board[old_point.row][old_point.col];
    self.board[old_point.row][old_point.col] = self.board[new_point.row][new_point.col];
    self.board[new_point.row][new_point.col] = temp;
  }

  fn swap_with_zero(&mut self, point: UPoint) {
    self.swap_node(self.zero_point, point);
    self.zero_point = point;
  }
}

fn main() -> Result<()> {
  let data = std::fs::read_to_string("levels.json")?;

  let contexts: Vec<InputContext> = serde_json::from_str(&data)?;

  for (i, context) in contexts.iter().enumerate() {
    if i == 0 {
      let mut solution = Solution::new(&context);
      println!("solutin:\n{}", solution);
      solution.process();
      break;
    }
  }

  Ok(())
}
