#[macro_use]
extern crate lazy_static;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct InputContext {
  board: Vec<Vec<i32>>,
  fixed: i32,
  size: i32,
  stage: i32,
}

#[derive(Debug)]
struct Context {
  path: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
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

fn main() -> Result<()> {
  let data = std::fs::read_to_string("levels.json")?;

  let contexts: Vec<InputContext> = serde_json::from_str(&data)?;

  for context in contexts.iter() {
    search(&context);
    break;
  }

  Ok(())
}

fn search(input_context: &InputContext) {
  println!("{:#?}", input_context);

  let mut board = input_context.board.clone();

  let result_bs = result_string(input_context.size);
  println!("result bs = {}", result_bs);

  let bs = board_to_string(&board);
  println!("bs = {}", bs);

  let zero = find_zero_point(&board);
  println!("zero = {:?}", zero);

  let context = Context { path: vec![] };

  let mut m = HashMap::new();
  m.insert(bs, context);

  let mut queue = VecDeque::new();
  queue.push_back(zero);

  while !queue.is_empty() {
    let zero = queue.pop_front().unwrap();
    let old_row = zero.row;
    let old_col = zero.col;
    for direction in DIRECTIONS.iter() {
      let new_row = (old_row as isize + direction.row) as usize;
      let new_col = (old_col as isize + direction.col) as usize;
      if is_valid_position(new_row, new_col, input_context.size) {
        swap_node(&mut board, old_row, old_col, new_row, new_col);

        let bs = board_to_string(&board);

        swap_node(&mut board, old_row, old_col, new_row, new_col);
      }
    }
  }
}

fn swap_node(
  board: &mut Vec<Vec<i32>>,
  old_row: usize,
  old_col: usize,
  new_row: usize,
  new_col: usize,
) {
  let temp = board[old_row][old_col];
  board[old_row][old_col] = board[new_row][new_col];
  board[new_row][new_col] = temp;
}

fn is_valid_position(row: usize, col: usize, size: i32) -> bool {
  let size = size as usize;
  row < size && col < size
}

fn find_zero_point(board: &Vec<Vec<i32>>) -> Point {
  for row in 0..board.len() {
    for col in 0..board[row].len() {
      if board[row][col] == 0 {
        return Point { row, col };
      }
    }
  }
  Point { row: 0, col: 0 }
}

fn result_string(size: i32) -> String {
  let mut result = String::new();
  let end = (size * size) as usize;
  for i in 0..end {
    result.push_str(&i.to_string());
    result.push_str(",");
  }
  result
}

fn board_to_string(board: &Vec<Vec<i32>>) -> String {
  let mut result = String::new();
  for rows in board.iter() {
    for item in rows.iter() {
      result.push_str(&item.to_string());
      result.push_str(",");
    }
  }
  result
}
