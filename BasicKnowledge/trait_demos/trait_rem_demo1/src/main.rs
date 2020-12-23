use std::ops::Rem;

#[derive(PartialEq, Debug)]
struct SplitSlice<'a, T: 'a> {
  slice: &'a [T],
}

impl<'a, T> Rem<usize> for SplitSlice<'a, T> {
  type Output = Self;

  fn rem(self, modulus: usize) -> Self::Output {
    let len = self.slice.len();
    let rem = len % modulus;
    let start = len - rem;
    SplitSlice {
      slice: &self.slice[start..],
    }
  }
}

fn main() {
  let a = SplitSlice {
    slice: &[0, 1, 2, 3, 4, 5, 6, 7],
  } % 3;
  println!("{:?}", a);
}
