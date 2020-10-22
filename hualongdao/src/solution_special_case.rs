use super::*;

impl Solution {
  pub(crate) fn check_special_case(&mut self) {
    self.specail_1();
  }

  fn specail_1(&mut self) {
    if self.fixed_point.row == self.start_row + 1 && self.fixed_point.col == self.start_col + 1 {
      let first_num = self.fixed - 1;
      let third_num = self.fixed - self.size;
      let second_num = third_num - 1;

      let first_dst_point = UPoint {
        row: self.fixed_point.row,
        col: self.fixed_point.col + 1,
      };
      let second_dst_point = UPoint {
        row: first_dst_point.row + 1,
        col: first_dst_point.col,
      };
      let third_dst_point = UPoint {
        row: first_dst_point.row + 2,
        col: first_dst_point.col,
      };

      self.move_number_to_dst(first_num, first_dst_point);
      self.move_number_to_dst(second_num, second_dst_point);
      self.move_number_to_dst(third_num, third_dst_point);

      let zero_dst = UPoint {
        row: first_dst_point.row - 1,
        col: first_dst_point.col,
      };
      let temp_fixed = vec![first_dst_point, second_dst_point, third_dst_point];
      self.move_zero_to_dst_with_temp_fixed(zero_dst, temp_fixed);
      self.swap_with_zero(first_dst_point);
      self.swap_with_zero(second_dst_point);
      self.swap_with_zero(third_dst_point);
    }
  }
}
