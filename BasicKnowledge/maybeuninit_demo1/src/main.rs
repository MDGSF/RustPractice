use std::mem::MaybeUninit;

fn main() {
  let mut x = MaybeUninit::<&i32>::uninit();
  unsafe {
    x.as_mut_ptr().write(&100);
  }
  let x = unsafe { x.assume_init() };
  println!("x = {}", x);
}
