extern crate tar;

use std::fs::File;
use tar::{Builder, Header};

fn main() {
  let file = File::create("foo.tar").unwrap();
  let mut a = Builder::new(file);

  a.append_file("file2.txt", &mut File::open("file3.txt").unwrap())
    .unwrap();

  let mut header = Header::new_gnu();
  header.set_size(4);
  header.set_cksum();
  header.set_mode(0o644);

  let data: &[u8] = &[1, 2, 3, 4];
  a.append_data(&mut header, "really/long/path/to/foo", data)
    .unwrap();

  a.finish().unwrap();
}
