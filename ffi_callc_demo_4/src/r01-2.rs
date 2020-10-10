use std::os::raw::c_int;

pub type SumSquareCB = unsafe extern "C" fn(c_int);

#[link(name = "ccode01")]
extern "C" {
  pub fn sum_square_cb01(a: c_int, b: c_int, cb: SumSquareCB);
}

fn main() {
  let mut sum = 0;

  unsafe {
    sum_square_cb01(3, 4, |r| sum += r);
  }

  println!("The result in callback function is: {}", sum);
}
