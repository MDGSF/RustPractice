use crate::*;
use anyhow::Result;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;

#[derive(Debug, Clone, Eq)]
struct Context {
  manhattan_distance: usize,
  board_str: String,
  path: Vec<String>,
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

fn number_to_point(num: usize, size: usize) -> Point {
  let row = (num - 1) / size;
  let col = (num - 1) % size;
  Point { row, col }
}

fn search_astar(input_context: &InputContext) {
  println!("{:#?}", input_context);

  let board_size = input_context.size as usize;
  let fixed_point = find_fixed_point(&input_context.board, input_context.fixed);

  let result_bs = result_string(input_context.size);
  println!("result bs = {}", result_bs);

  let bs = board_to_string(&input_context.board);

  let context = Context {
    manhattan_distance: calc_manhattan_distance(&input_context.board),
    board_str: bs.clone(),
    path: vec![],
  };

  let mut m = HashSet::new();
  m.insert(bs);

  let mut heap = BinaryHeap::new();
  heap.push(context);

  while !heap.is_empty() {
    let context = heap.pop().unwrap();
    let bs = &context.board_str;
    let mut board = string_to_board(&bs, board_size);
    let zero = find_zero_point(&board);
    let path = &context.path;
    //println!("bs = {}", bs);
    //println!("context = {:?}", context);
    //println!("board = {:?}", board);
    //println!("zero = {:?}", zero);
    //println!("path = {:?}", path);

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

          println!("board = {:?}", board);
          println!("path = {:?}", new_path.join(""));

          return;
        }
        if !m.contains(&new_bs) {
          let mut new_path = path.clone();
          new_path.push(direction.name.to_string());
          let new_context = Context {
            manhattan_distance: calc_manhattan_distance(&board),
            board_str: new_bs.clone(),
            path: new_path,
          };
          m.insert(new_bs.clone());
          heap.push(new_context);
        }

        swap_node(&mut board, old_row, old_col, new_row, new_col);
      }
    }
  }
}

pub fn astar() -> Result<()> {
  let data = std::fs::read_to_string("levels.json")?;

  let contexts: Vec<InputContext> = serde_json::from_str(&data)?;

  for (i, context) in contexts.iter().enumerate() {
    if i == 1 {
      search_astar(&context);
      break;
    }
  }

  Ok(())
}
