fn main() {
  test1();
  test2();
  test3();
}

fn test1() {
  let a = [1, 2, 3];

  let mut iter = a.iter();

  assert_eq!(Some(&1), iter.next());
  assert_eq!(Some(&2), iter.next());
  assert_eq!(Some(&3), iter.next());

  assert_eq!(None, iter.next());
  assert_eq!(None, iter.next());
  assert_eq!(None, iter.next());

  println!("test1 a = {:?}", a);
}

fn test2() {
  let x = &mut [1, 2, 4];
  for elem in x.iter_mut() {
    *elem += 2;
  }
  assert_eq!(x, &[3, 4, 6]);
}

fn test3() {
  let a = vec![1, 2, 3];

  let mut iter = a.into_iter();

  assert_eq!(Some(1), iter.next());
  assert_eq!(Some(2), iter.next());
  assert_eq!(Some(3), iter.next());

  assert_eq!(None, iter.next());
  assert_eq!(None, iter.next());
  assert_eq!(None, iter.next());

  // println!("{:?}", a);
}
