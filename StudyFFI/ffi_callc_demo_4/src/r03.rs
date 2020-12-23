use std::ffi::c_void;
use std::os::raw::c_int;

pub type SumSquareCB = unsafe extern "C" fn(c_int, *mut c_void);

#[link(name = "ccode03")]
extern "C" {
  pub fn sum_square_cb03(a: c_int, b: c_int, cb: SumSquareCB, user_data: *mut c_void);
}

pub unsafe extern "C" fn cb_func(result: c_int, user_data: *mut c_void) {
  let data = &mut *(user_data as *mut SumRecord);
  data.sum += result;
  data.elem_number += 1;
}

#[derive(Debug, Default, Clone, PartialEq)]
struct SumRecord {
  sum: c_int,
  elem_number: usize,
}

fn main() {
  let mut sum = SumRecord::default();

  unsafe {
    sum_square_cb03(3, 4, cb_func, &mut sum as *mut SumRecord as *mut c_void);
  }

  println!("The sum is {:?}", sum);
}
