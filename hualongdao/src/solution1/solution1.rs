use crate::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;

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

    let zero_point = board.find_num(0).unwrap();

    let max_number = input_context.size * input_context.size;

    let mut fixed_points: HashSet<Point> = HashSet::new();

    let fixed_point = if input_context.fixed > 0 {
      let fixed_point = board.number_to_point(input_context.fixed);
      fixed_points.insert(fixed_point);
      fixed_point
    } else {
      Point::new(0, 0)
    };

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

  pub fn search_bfs(&mut self) -> Option<String> {
    info!("search bfs start {}", self);
    let result_bs = result_string(self.size);
    println!("result bs = {}", result_bs);

    let bs = self.board.encode_to_string();

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
      let mut board = Board::decode_from_string(&bs, self.size);
      let zero = board.find_num(0).unwrap();
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

        let t1 = is_valid_position(new_row, new_col, self.size);
        let t2 = self.is_fixed_upoint(&Point::new(new_row, new_col));
        if t1 && !t2 {
          board.swap_points(Point::new(old_row, old_col), Point::new(new_row, new_col));

          let new_bs = board.encode_to_string();
          if new_bs == result_bs {
            info!("found result");

            let mut new_path = path.clone();
            new_path.push(direction.name.to_string());

            let result = new_path.join("");
            info!("path = {:?}", result);

            return Some(result);
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

          board.swap_points(Point::new(old_row, old_col), Point::new(new_row, new_col));
        }
      } // for
    } // while
    None
  }

  pub fn is_fixed_upoint(&self, point: &Point) -> bool {
    self.fixed_points.contains(point)
  }
}

impl fmt::Display for Solution1 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut result = String::new();
    result.push_str(&format!("[solution1] ---------------\n"));
    result.push_str(&format!("[solution1] stage: {}\n", self.stage));
    result.push_str(&format!(
      "[solution1] size: {} * {} = {}\n",
      self.size, self.size, self.max_number,
    ));

    result.push_str(&format!("{}", self.board));

    result.push_str(&format!(
      "[solution1] fixed: {}, {:?}\n",
      self.fixed, self.fixed_point
    ));

    let mut fixed_points: Vec<_> = self.fixed_points.iter().collect();
    fixed_points.sort();
    result.push_str(&format!("[solution1] fixed_points: {:?}\n", fixed_points));
    result.push_str(&format!("[solution1] zero_point: {:?}\n", self.zero_point));
    result.push_str(&format!("[solution1] result: {}\n", self.result.join("")));
    result.push_str(&format!("[solution1] ---------------\n"));
    write!(f, "{}", result)
  }
}

#[derive(Debug, Clone)]
struct Context {
  board_str: String,
  path: Vec<String>,
}

fn is_valid_position(row: usize, col: usize, size: usize) -> bool {
  let size = size as usize;
  row < size && col < size
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
