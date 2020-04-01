#![feature(range_is_empty)]
fn main() {
  let mut v = vec![3, 2, 5, 1, 4];
  println!("before sort, v = {:?}", v);

  bubble_sort(&mut v);
  println!("after sort, v = {:?}", v);
}

fn bubble_sort<T>(v: &mut [T])
where
  T: PartialOrd + Copy + std::fmt::Display,
{
  for i in 0..v.len() {
    for j in i..(v.len() - 1) {
      if v[j] > v[j + 1] {
        v.swap(j, j + 1);
      }
    }
  }
}

fn bubble_sort2<T>(v: &mut [T])
where
  T: PartialOrd + Copy,
{
  let mut i = 0;
  while i < v.len() {
    let mut j = v.len() - 1;
    while j > i {
      if v[j - 1] > v[j] {
        v.swap(j - 1, j);
      }
      j -= 1;
    }
    i += 1;
  }
}
