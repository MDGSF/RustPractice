fn main() {
  test1();
  test2();
  test3();
}

fn test1() {
  let a = [1, 2, 3];
  let iter = a.iter();
  println!("{:?}", iter.size_hint());
}

fn test2() {
  let iter = (0..10).filter(|x| x % 2 == 0);
  println!("{:?}", iter.size_hint());

  let iter = (0..10).filter(|x| x % 2 == 0).chain(15..20);
  println!("{:?}", iter.size_hint());
}

fn test3() {
  let iter = 0..;
  println!("{:?}", iter.size_hint());
  assert_eq!((usize::max_value(), None), iter.size_hint());
}
