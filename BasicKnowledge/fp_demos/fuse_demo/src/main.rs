// https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fuse

struct Alternate {
  state: i32,
}

impl Iterator for Alternate {
  type Item = i32;

  fn next(&mut self) -> Option<i32> {
    let val = self.state;
    self.state = self.state + 1;

    if val % 2 == 0 {
      Some(val)
    } else {
      None
    }
  }
}

fn main() {
  let mut iter = Alternate { state: 0 };

  // we can see our iterator going back and forth
  assert_eq!(iter.next(), Some(0));
  assert_eq!(iter.next(), None);
  assert_eq!(iter.next(), Some(2));
  assert_eq!(iter.next(), None);

  // however, once we fuse it...
  let mut iter = iter.fuse();

  assert_eq!(iter.next(), Some(4));
  assert_eq!(iter.next(), None);

  // it will always return `None` after the first time.
  assert_eq!(iter.next(), None);
  assert_eq!(iter.next(), None);
  assert_eq!(iter.next(), None);
  assert_eq!(iter.next(), None);
}
