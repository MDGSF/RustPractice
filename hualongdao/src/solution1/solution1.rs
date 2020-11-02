use crate::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

pub struct Solution1 {
  board: Board,                 // board 是个正方形
  fixed: usize,                 // 固定点的数字
  size: usize,                  // board 的边长
  stage: usize,                 // 第几关
  fixed_point: Point,           // fixed 的行列位置
  max_number: usize,            // 最大数字 size * size
  fixed_points: HashSet<Point>, // 当前不能被移动的点
  zero_point: Point,            // 空格的位置
  result: Vec<String>,          // 保存最后的结果，空格的移动命令，L R U D
}

impl Solution1 {
  pub fn new(input_context: &InputContext) -> Solution1 {
    let board = Board::new(input_context.board.clone());

    let fixed_point = board.number_to_point(input_context.fixed);

    let zero_point = board.find_num(0).unwrap();

    let max_number = input_context.size * input_context.size;

    let mut fixed_points: HashSet<Point> = HashSet::new();
    fixed_points.insert(fixed_point);

    Solution1 {
      board: board,
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
}

#[derive(Debug, Clone)]
struct Context {
  board_str: String,
  path: Vec<String>,
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

pub fn search_bfs(input_context: &InputContext) {
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
