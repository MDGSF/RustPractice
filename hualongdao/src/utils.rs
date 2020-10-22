use super::*;

/// 计算两个点之间的曼哈顿距离
pub fn calc_two_point_manhattan_distance(p1: UPoint, p2: UPoint) -> usize {
  (p1.row as isize - p2.row as isize).abs() as usize
    + (p1.col as isize - p2.col as isize).abs() as usize
}

/// 计算 num 数字在以 size 为边长的正方形上的位置。
///
/// # Example
/// num = 4, size = 3, output = UPoint { 1, 0 }
/// 1 2 3
/// 4 5 6
/// 7 8 9
pub fn number_to_square_point(num: usize, size: usize) -> UPoint {
  let row = (num - 1) / size;
  let col = (num - 1) % size;
  UPoint { row, col }
}

/// 在 board 上遍历查找指定的 num 数字，返回其下标。
pub fn find_num_point_in_board(board: &Vec<Vec<usize>>, num: usize) -> UPoint {
  for row in 0..board.len() {
    for col in 0..board[row].len() {
      if board[row][col] == num {
        return UPoint { row, col };
      }
    }
  }
  UPoint { row: 0, col: 0 }
}

/// 交换 board 上面的 point1 和 point2 两个点的值。
pub fn swap_two_points(board: &mut Vec<Vec<usize>>, point1: UPoint, point2: UPoint) {
  let temp = board[point1.row][point1.col];
  board[point1.row][point1.col] = board[point2.row][point2.col];
  board[point2.row][point2.col] = temp;
}
