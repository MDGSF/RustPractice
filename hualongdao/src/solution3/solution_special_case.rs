use super::*;

impl Solution {
  pub(super) fn find_special_case(&mut self) -> bool {
    if self.special_1_condition() {
      self.specail_1_process();
      return true;
    }

    if self.special_2_condition() {
      self.specail_2_process();
      return true;
    }

    false
  }

  fn special_1_condition(&self) -> bool {
    let p1_num = self.fixed - 1;
    let p3_num = self.fixed - self.size;
    let p2_num = p3_num - 1;

    let frow = self.fixed_point.row;
    let fcol = self.fixed_point.col;
    let p1 = Point::new(frow, fcol - 1);
    let p2 = Point::new(frow - 1, fcol - 1);
    let p3 = Point::new(frow - 1, fcol);

    if self.fixed_point.row == self.start_row + 1 && self.fixed_point.col == self.start_col + 1 {
      if self.board[p1] == p1_num && self.board[p2] == p2_num && self.board[p3] == p3_num {
        return false;
      } else {
        return true;
      }
    }
    false
  }

  // x x  x  x  x
  // x p2 p3 p4
  // x p1 f  p5
  // x x  x  p6
  // x x  x  p7
  fn specail_1_process(&mut self) {
    let p1_num = self.fixed - 1;
    let p3_num = self.fixed - self.size;
    let p2_num = p3_num - 1;

    let frow = self.fixed_point.row;
    let fcol = self.fixed_point.col;
    let p1 = Point::new(frow, fcol - 1);
    let p2 = Point::new(frow - 1, fcol - 1);
    let p3 = Point::new(frow - 1, fcol);
    let p4 = Point::new(frow - 1, fcol + 1);
    let p5 = Point::new(frow, fcol + 1);
    let p6 = Point::new(frow + 1, fcol + 1);

    self.move_number_to_dst(p1_num, p4);
    self.move_number_to_dst_with_temp_fixed(p2_num, p5, vec![p4]);
    self.move_number_to_dst_with_temp_fixed(p3_num, p6, vec![p4, p5]);
    self.move_zero_to_dst_with_temp_fixed(p3, vec![p4, p5, p6]);
    self.move_zero_with_paths(vec![p4, p5, p6]);
    self.move_zero_to_dst_with_temp_fixed(p2, vec![p3, p4, p5]);
    self.move_zero_with_paths(vec![p3, p4, p5]);
    self.move_zero_to_dst_with_temp_fixed(p1, vec![p2, p3, p4]);
    self.move_zero_with_paths(vec![p2, p3, p4]);
  }
}
