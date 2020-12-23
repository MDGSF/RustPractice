// take 接收一个可变引用，用默认值代替旧值，返回旧值。
use std::mem;

fn main() {
  println!("Hello, world!");
  test1();
  test2();
  test3();
}

fn test1() {
  let mut v: Vec<i32> = vec![1, 2];
  let old_v = mem::take(&mut v);
  println!("old_v = {:?}", old_v);
  println!("v = {:?}", v);
}

struct Buffer<T> {
  buf: Vec<T>,
}

impl<T> Buffer<T> {
  fn get_and_reset(&mut self) -> Vec<T> {
    mem::take(&mut self.buf)
  }
}

fn test2() {
  let mut buffer = Buffer { buf: vec![0, 1] };
  println!("len = {}", buffer.buf.len());

  println!("old buf = {:?}", buffer.get_and_reset());
  println!("len = {}", buffer.buf.len());
}

fn test3() {
  let mut i = 3;
  let old_i = mem::take(&mut i);
  println!("old_i = {}", old_i);
  println!("i = {}", i);
}
