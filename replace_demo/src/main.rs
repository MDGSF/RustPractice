// 把 src 的值放到 dest 中，返回 dest 的旧值。
use std::mem;

fn main() {
  println!("Hello, world!");
  test1();
  test2();
}

fn test1() {
  let mut v: Vec<i32> = vec![1, 2];
  let old_v = mem::replace(&mut v, vec![3, 4, 5, 6]);
  println!("old_v = {:?}", old_v);
  println!("v = {:?}", v);
}

struct Buffer<T> {
  buf: Vec<T>,
}

impl<T> Buffer<T> {
  fn replace_index(&mut self, i: usize, v: T) -> T {
    mem::replace(&mut self.buf[i], v)
  }
}

fn test2() {
  let mut buffer = Buffer { buf: vec![0, 1] };
  println!("buffer = {:?}", buffer.buf);

  let old_v = buffer.replace_index(0, 2);
  println!("old_v = {}", old_v);
  println!("buffer = {:?}", buffer.buf);
}
