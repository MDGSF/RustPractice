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

#[derive(Debug, Clone)]
struct Context {
  board_str: String,
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

  for (i, context) in contexts.iter().enumerate() {
    if i == 1 {
      search(&context);
      break;
    }
  }

  Ok(())
}

fn search(input_context: &InputContext) {
  println!("{:#?}", input_context);

  let board_size = input_context.size as usize;
  let fixed_point = find_fixed_point(&input_context.board, input_context.fixed);

  let result_bs = result_string(input_context.size);
  println!("result bs = {}", result_bs);

  let bs = board_to_string(&input_context.board);

  let context = Context {
    board_str: bs.clone(),
    path: vec![],
  };

  let mut m = HashMap::new();
  m.insert(bs.clone(), context);

  let mut queue = VecDeque::new();
  queue.push_back(bs);

  while !queue.is_empty() {
    let bs = queue.pop_front().unwrap();
    let context = m.get(&bs).unwrap().clone();
    let mut board = string_to_board(&bs, board_size);
    let zero = find_zero_point(&board);
    let path = &context.path;
    // println!("bs = {}", bs);
    // println!("context = {:?}", context);
    // println!("board = {:?}", board);
    // println!("zero = {:?}", zero);
    // println!("path = {:?}", path);

    let old_row = zero.row;
    let old_col = zero.col;
    for direction in DIRECTIONS.iter() {
      let new_row = (old_row as isize + direction.row) as usize;
      let new_col = (old_col as isize + direction.col) as usize;
      if is_valid_position(new_row, new_col, input_context.size)
        && (new_row != fixed_point.row || new_col != fixed_point.col)
      {
        swap_node(&mut board, old_row, old_col, new_row, new_col);

        let new_bs = board_to_string(&board);
        if new_bs == result_bs {
          println!("found result");

          let mut new_path = path.clone();
          new_path.push(direction.name.to_string());

          println!("path = {:?}", new_path.join(""));

          return;
        }
        if !m.contains_key(&new_bs) {
          let mut new_path = path.clone();
          new_path.push(direction.name.to_string());
          let new_context = Context {
            board_str: new_bs.clone(),
            path: new_path,
          };
          m.insert(new_bs.clone(), new_context);
          queue.push_back(new_bs);
        }

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

fn find_fixed_point(board: &Vec<Vec<i32>>, fixed: i32) -> Point {
  for row in 0..board.len() {
    for col in 0..board[row].len() {
      if board[row][col] == fixed {
        return Point { row, col };
      }
    }
  }
  Point { row: 0, col: 0 }
}

fn result_string(size: i32) -> String {
  let mut result = String::new();
  let end = (size * size) as usize;
  for i in 1..end {
    result.push_str(&i.to_string());
    result.push_str(",");
  }
  result.push_str("0,");
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

fn string_to_board(board_str: &str, size: usize) -> Vec<Vec<i32>> {
  let temp: Vec<i32> = board_str
    .split(',')
    .filter(|x| x.len() > 0)
    .map(|x| x.parse::<i32>().unwrap())
    .collect();
  let mut result = vec![vec![0i32; size]; size];
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
