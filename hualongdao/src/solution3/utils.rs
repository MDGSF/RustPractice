use super::*;

/// 计算两个点之间的曼哈顿距离
pub fn calc_two_point_manhattan_distance(p1: Point, p2: Point) -> usize {
  (p1.row as isize - p2.row as isize).abs() as usize
    + (p1.col as isize - p2.col as isize).abs() as usize
}
