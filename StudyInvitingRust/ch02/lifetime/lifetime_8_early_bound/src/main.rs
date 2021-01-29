struct Buffer<'a> {
  buf: &'a [u8],
  pos: usize,
}

impl<'a: 'b, 'b> Buffer<'a> {
  fn new(b: &'a [u8]) -> Buffer {
    Buffer { buf: b, pos: 0 }
  }

  // 'a 是成员 buf 的生命周期
  // 'b 是对象的生命周期
  // 所以 'a > 'b
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
  //let b1 = &(buf.read_bytes().to_owned());
  let b2 = buf.read_bytes();
  print(b1, b2);
}
