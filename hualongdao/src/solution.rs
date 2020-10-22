use super::*;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;

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

pub struct Solution {
  pub board: Vec<Vec<usize>>,        // board 是个正方形
  pub fixed: usize,                  // 固定点的数字
  pub size: usize,                   // board 的边长
  pub stage: usize,                  // 第几关
  pub fixed_point: UPoint,           // fixed 的行列位置
  pub max_number: usize,             // 最大数字 size * size
  pub fixed_points: HashSet<UPoint>, // 当前不能被移动的点
  pub zero_point: UPoint,            // 空格的位置
  pub result: Vec<String>,           // 保存最后的结果，空格的移动命令，L R U D
  pub start_row: usize,
  pub start_col: usize,
  pub end_row: usize,
  pub end_col: usize,
}

impl Solution {
  pub fn new(input_context: &InputContext) -> Solution {
    let fixed_point = number_to_square_point(input_context.fixed, input_context.size);
    let max_number = input_context.size * input_context.size;
    let mut fixed_points: HashSet<UPoint> = HashSet::new();
    fixed_points.insert(fixed_point);
    let zero_point = find_num_point_in_board(&input_context.board, 0);
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
      start_row: 0,
      start_col: 0,
      end_row: input_context.size - 1,
      end_col: input_context.size - 1,
    }
  }

  pub fn process(&mut self) {
    for num in 1..self.max_number {
      self.check_special_case();
      self.move_number(num);
    }
  }

  pub(crate) fn move_number(&mut self, num: usize) {
    println!("\n----------------");
    println!("move start num = {}", num);
    println!("{}", self);

    let num_point = number_to_square_point(num, self.size);

    if num % self.size == 0 {
      self.move_row_last_number(num);
    } else if num_point.row == self.size - 1 {
      self.move_col_last_number(num);
    } else {
      self.move_basic_number(num);
    }

    self.fixed_points.insert(num_point);
  }

  fn move_row_last_number(&mut self, num: usize) {
    let src_point = find_num_point_in_board(&self.board, num);
    let dst_point = number_to_square_point(num, self.size);
    if src_point == dst_point {
      println!("num = {}, no need to move", num);
      return;
    }
    let mut pre_dst_point = dst_point;
    pre_dst_point.col += 1;
    self.move_number_from_src_to_dst(num, src_point, pre_dst_point);

    // TODO
    if self.zero_point == dst_point {
      self.swap_with_zero(pre_dst_point);
    }
  }

  fn move_col_last_number(&mut self, num: usize) {
    let src_point = find_num_point_in_board(&self.board, num);
    let dst_point = number_to_square_point(num, self.size);
    if src_point == dst_point {
      println!("num = {}, no need to move", num);
      return;
    }
    let mut pre_dst_point = dst_point;
    pre_dst_point.col += 1;
    self.move_number_from_src_to_dst(num, src_point, pre_dst_point);

    // TODO
    if self.zero_point == dst_point {
      self.swap_with_zero(pre_dst_point);
    }
  }

  pub(crate) fn move_basic_number(&mut self, num: usize) {
    let src_point = find_num_point_in_board(&self.board, num);
    let dst_point = number_to_square_point(num, self.size);
    if src_point == dst_point {
      println!("num = {}, no need to move", num);
      return;
    }
    self.move_number_from_src_to_dst(num, src_point, dst_point);
  }

  pub(crate) fn move_number_to_dst(&mut self, num: usize, dst_point: UPoint) {
    let src_point = find_num_point_in_board(&self.board, num);
    self.move_number_from_src_to_dst(num, src_point, dst_point);
  }

  pub(crate) fn move_number_from_src_to_dst(
    &mut self,
    num: usize,
    src_point: UPoint,
    dst_point: UPoint,
  ) {
    if src_point == dst_point {
      println!("num = {}, no need to move", num);
      return;
    } else {
      println!("move {} from {:?} to {:?}", num, src_point, dst_point);
    }

    let num_paths = self.find_path(num, src_point, dst_point);
    if num_paths.is_none() {
      println!("{}", self);
      panic!("find special case");
    }
    let num_paths = num_paths.unwrap();
    // println!("num_paths = {:?}", num_paths);

    let mut num_point = src_point;
    for path_point in num_paths {
      // 先把 0 移动到要移动的数字前面
      self.move_zero_to_dst_with_temp_fixed(path_point, vec![num_point]);

      // 把数字向前移动一步
      self.swap_with_zero(num_point);
      num_point = path_point;
    }
  }

  // 把 self.zero_point 移动到 dst_point 的位置
  // 并在移动的过程中把 temp_fixed 数组中的 point 设置为固定点
  pub(crate) fn move_zero_to_dst_with_temp_fixed(
    &mut self,
    dst_point: UPoint,
    temp_fixed: Vec<UPoint>,
  ) {
    if self.zero_point == dst_point {
      return;
    }

    for &point in temp_fixed.iter() {
      self.fixed_points.insert(point);
    }

    self.move_zero_to_dst(dst_point);

    for point in temp_fixed.iter() {
      self.fixed_points.remove(&point);
    }
  }

  // 把 self.zero_point 移动到 dst_point 的位置
  pub(crate) fn move_zero_to_dst(&mut self, dst_point: UPoint) {
    if self.zero_point == dst_point {
      return;
    }

    let zero_paths = self.find_path(0, self.zero_point, dst_point);
    if zero_paths.is_none() {
      println!("{}", self);
      panic!("find special case");
    }
    let zero_paths = zero_paths.unwrap();

    self.move_zero_with_paths(zero_paths);
  }

  // 让 self.zero_point 沿着 zero_paths 移动
  pub(crate) fn move_zero_with_paths(&mut self, zero_paths: Vec<UPoint>) {
    for &path_point in zero_paths.iter() {
      self.swap_with_zero(path_point);
    }
  }

  // 查找从 src_point 到 dst_point 的移动路径
  // 返回的移动路径，不包含 src_point, 包含 dst_point
  // 移动时，无法跨越固定点
  pub(crate) fn find_path(
    &mut self,
    _num: usize,
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
          s.insert(new_upoint);
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

  // 判断 point 点是否在正方形内
  pub(crate) fn is_valid_ipoint(&self, point: &IPoint) -> bool {
    let size = self.size as isize;
    point.row >= 0 && point.row < size && point.col >= 0 && point.col < size
  }

  // 判断 point 是否是固定点
  pub(crate) fn is_fixed_upoint(&self, point: &UPoint) -> bool {
    self.fixed_points.contains(point)
  }

  // 交换 point1 和 point2 两个点的值
  pub(crate) fn swap_node(&mut self, point1: UPoint, point2: UPoint) {
    swap_two_points(&mut self.board, point1, point2);
  }

  // 1. 交换 zero_point 和 point 的值
  // 2. 并更新 self.zero_point 的位置
  // 3. 记录 self.zero_point 移动的路径
  pub(crate) fn swap_with_zero(&mut self, point: UPoint) {
    self.record_zero_point_move_poing(point);
    self.swap_node(self.zero_point, point);
    self.zero_point = point;
  }

  // 记录 self.zero_point 移动的路径
  pub(crate) fn record_zero_point_move_poing(&mut self, point: UPoint) {
    for direction in DIRECTIONS.iter() {
      let ipoint = self.zero_point + direction;
      let upoint = UPoint::newi(ipoint);
      if upoint == point {
        self.result.push(direction.name.to_string());
      }
    }
  }
}
