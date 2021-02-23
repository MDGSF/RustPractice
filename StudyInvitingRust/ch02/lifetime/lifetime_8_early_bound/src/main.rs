struct Buffer<'a> {
  buf: &'a [u8],
  pos: usize,
}

// 'a 可以认为是 main 函数中 v 的生命周期
// 'b 可以认为是 main 函数中 buf 这个对象的生命周期
// 'a 的意思是说 Buffer 这个类 new 出来的对象的生命周期不能超过 'a
impl<'a: 'b, 'b> Buffer<'a> {
  fn new(b: &'a [u8]) -> Buffer {
    Buffer { buf: b, pos: 0 }
  }

  fn read_bytes(&'b mut self) -> &'a [u8] {
    self.pos += 3;
    &self.buf[self.pos - 3..self.pos]
  }
}

fn print(b1: &[u8], b2: &[u8]) {
  println!("{:#?}, {:#?}", b1, b2);
}

fn main() {
  let v = vec![1, 2, 3, 4, 5, 6];
  let mut buf = Buffer::new(&v);
  let b1 = buf.read_bytes();
  let b2 = buf.read_bytes();

  // 即使把 buf 删除了，b1 和 b2 也不受影响
  drop(buf);

  print(b1, b2);
}
