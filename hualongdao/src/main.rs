#[macro_use]
extern crate lazy_static;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct InputContext {
  board: Vec<Vec<usize>>,
  fixed: usize,
  size: usize,
  stage: usize,
}

#[derive(Debug, Clone, Eq)]
struct Context {
  position: Point,
  manhattan_distance: usize,
  path: Vec<Point>,
}

impl Ord for Context {
  fn cmp(&self, other: &Self) -> Ordering {
    other.manhattan_distance.cmp(&self.manhattan_distance)
  }
}

impl PartialOrd for Context {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl PartialEq for Context {
  fn eq(&self, other: &Self) -> bool {
    self.manhattan_distance == other.manhattan_distance
  }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Point {
  row: usize,
  col: usize,
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

fn is_valid_position(row: usize, col: usize, size: usize) -> bool {
  let size = size as usize;
  row < size && col < size
}

fn find_zero_point(board: &Vec<Vec<usize>>) -> Point {
  for row in 0..board.len() {
    for col in 0..board[row].len() {
      if board[row][col] == 0 {
        return Point { row, col };
      }
    }
  }
  Point { row: 0, col: 0 }
}

fn find_fixed_point(board: &Vec<Vec<usize>>, fixed: usize) -> Point {
  for row in 0..board.len() {
    for col in 0..board[row].len() {
      if board[row][col] == fixed {
        return Point { row, col };
      }
    }
  }
  Point { row: 0, col: 0 }
}

fn result_string(size: usize) -> String {
  let mut result = String::new();
  let end = (size * size) as usize;
  for i in 1..end {
    result.push_str(&i.to_string());
    result.push_str(",");
  }
  result.push_str("0,");
  result
}

fn board_to_string(board: &Vec<Vec<usize>>) -> String {
  let mut result = String::new();
  for rows in board.iter() {
    for item in rows.iter() {
      result.push_str(&item.to_string());
      result.push_str(",");
    }
  }
  result
}

fn string_to_board(board_str: &str, size: usize) -> Vec<Vec<usize>> {
  let temp: Vec<usize> = board_str
    .split(',')
    .filter(|x| x.len() > 0)
    .map(|x| x.parse::<usize>().unwrap())
    .collect();
  let mut result = vec![vec![0_usize; size]; size];
  let mut row = 0;
  let mut col = 0;
  for v in temp {
    result[row][col] = v;
    col += 1;
    if col == size {
      row += 1;
      col = 0;
    }
  }
  result
}

fn calc_manhattan_distance(board: &Vec<Vec<usize>>) -> usize {
  let mut result: usize = 0;
  for row in 0..board.len() {
    for col in 0..board[row].len() {
      if board[row][col] > 0 {
        let expect_point = number_to_point(board[row][col], board.len());
        let cur_distance = (row as isize - expect_point.row as isize).abs()
          + (col as isize - expect_point.col as isize).abs();
        result += cur_distance as usize;
      }
    }
  }
  result
}

fn calc_two_point_manhattan_distance(p1: Point, p2: Point) -> usize {
  (p1.row as isize - p2.row as isize).abs() as usize
    + (p1.col as isize - p2.col as isize).abs() as usize
}

fn number_to_point(num: usize, size: usize) -> Point {
  let row = (num - 1) / size;
  let col = (num - 1) % size;
  Point { row, col }
}

fn find_num_point(board: &Vec<Vec<usize>>, num: usize) -> Point {
  for row in 0..board.len() {
    for col in 0..board[row].len() {
      if board[row][col] == num {
        return Point { row, col };
      }
    }
  }
  Point { row: 0, col: 0 }
}

fn pretty_show_board(board: &Vec<Vec<usize>>) {
  for row in 0..board.len() {
    print!("[ ");
    for col in 0..board[row].len() {
      print!("{} ", board[row][col]);
    }
    println!("]");
  }
}

fn find_num_path_bfs(
  board: &mut Vec<Vec<usize>>,
  num: usize,
  src_point: Point,
  dst_point: Point,
  fixed_points: &HashSet<Point>,
) -> Vec<Point> {
  let size = board.len();

  let context = Context {
    position: src_point,
    manhattan_distance: calc_two_point_manhattan_distance(src_point, dst_point),
    path: vec![],
  };

  let size = board.len();
  let mut s = HashSet::new();
  s.insert(src_point);

  let mut heap = BinaryHeap::new();
  heap.push(context);

  while !heap.is_empty() {
    let context = heap.pop().unwrap();
    let point = context.position;

    let old_row = point.row;
    let old_col = point.col;
    for direction in DIRECTIONS.iter() {
      let new_row = (old_row as isize + direction.row) as usize;
      let new_col = (old_col as isize + direction.col) as usize;
      let new_point = Point {
        row: new_row,
        col: new_col,
      };

      if new_point == dst_point {
        let mut new_path = context.path.clone();
        new_path.push(new_point);
        return new_path;
      }

      if is_valid_position(new_row, new_col, size) && !s.contains(&new_point) {
        let mut new_path = context.path.clone();
        new_path.push(new_point);
        let new_context = Context {
          position: new_point,
          manhattan_distance: calc_two_point_manhattan_distance(new_point, dst_point),
          path: new_path,
        };
        heap.push(new_context);
      }
    }
  }

  vec![]
}

fn move_number_to_dst(
  board: &mut Vec<Vec<usize>>,
  num: usize,
  src_point: Point,
  dst_point: Point,
  fixed_points: &HashSet<Point>,
) {
  if src_point == dst_point {
    println!("num = {}, no need to move", num);
    return;
  }

  let num_paths = find_num_path_bfs(board, num, src_point, dst_point, fixed_points);
  println!("num_paths = {:?}", num_paths);
}

fn move_basic_number(
  board: &mut Vec<Vec<usize>>,
  num: usize,
  size: usize,
  fixed_points: &HashSet<Point>,
) {
  let src_point = find_num_point(&board, num);
  let dst_point = number_to_point(num, size);
  move_number_to_dst(board, num, src_point, dst_point, fixed_points);
}

fn move_row_last_number(
  board: &mut Vec<Vec<usize>>,
  num: usize,
  size: usize,
  fixed_points: &HashSet<Point>,
) {
  let src_point = find_num_point(&board, num);
  let mut dst_point = number_to_point(num, size);
  dst_point.row += 1;
  move_number_to_dst(board, num, src_point, dst_point, fixed_points);

  // TODO
}

fn move_col_last_number(
  board: &mut Vec<Vec<usize>>,
  num: usize,
  size: usize,
  fixed_points: &HashSet<Point>,
) {
  let src_point = find_num_point(&board, num);
  let mut dst_point = number_to_point(num, size);
  dst_point.col += 1;
  move_number_to_dst(board, num, src_point, dst_point, fixed_points);

  // TODO
}

fn move_number(
  board: &mut Vec<Vec<usize>>,
  num: usize,
  size: usize,
  fixed_points: &HashSet<Point>,
) {
  let zero = find_zero_point(&board);
  println!("num = {}", num);

  let num_point = number_to_point(num, size);

  if num % size == 0 {
    move_row_last_number(board, num, size, fixed_points);
  } else if num_point.row == size - 1 {
    move_col_last_number(board, num, size, fixed_points);
  } else {
    move_basic_number(board, num, size, fixed_points);
  }
}

fn process_no_fix(input_context: &InputContext) {
  let mut board = input_context.board.clone();
  let size = board.len();
  pretty_show_board(&board);

  let mut fixed_points: HashSet<Point> = HashSet::new();
  let max_number = input_context.size * input_context.size;
  for num in 1..max_number {
    move_number(&mut board, num, input_context.size, &fixed_points);

    let num_point = number_to_point(num, size);
    fixed_points.insert(num_point);
  }
}

fn process(input_context: &InputContext) {
  println!("{:#?}", input_context);

  let fixed_point = number_to_point(input_context.fixed, input_context.size);
  println!("fixed_point = {:?}", fixed_point);

  process_no_fix(input_context);
}

fn main() -> Result<()> {
  let data = std::fs::read_to_string("levels.json")?;

  let contexts: Vec<InputContext> = serde_json::from_str(&data)?;

  for (i, context) in contexts.iter().enumerate() {
    if i == 0 {
      process(&context);
      break;
    }
  }

  Ok(())
}
