struct Buffer {
  buf: Vec<u8>,
  pos: usize,
}

impl Buffer {
  fn new() -> Buffer {
    Buffer {
      buf: vec![1, 2, 3, 4, 5, 6],
      pos: 0,
    }
  }

  fn read_bytes<'a>(&'a mut self) -> &'a [u8] {
    self.pos += 3;
    &self.buf[self.pos - 3..self.pos]
  }
}

fn print(b1: &[u8], b2: &[u8]) {
  println!("{:#?}, {:#?}", b1, b2);
}

fn main() {
  let mut buf = Buffer::new();
  let b1 = buf.read_bytes();
  //let b1 = &(buf.read_bytes().to_owned());
  let b2 = buf.read_bytes();
  print(b1, b2);
}
