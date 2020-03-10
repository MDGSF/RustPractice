fn main() {
  test1();
  test2();
}

fn test1() {
  let a1 = [1, 2, 3];
  let a2 = [4, 5, 6];

  let mut iter = a1.iter().zip(a2.iter());

  assert_eq!(iter.next(), Some((&1, &4)));
  assert_eq!(iter.next(), Some((&2, &5)));
  assert_eq!(iter.next(), Some((&3, &6)));
  assert_eq!(iter.next(), None);
}

fn test2() {
  let enumerate: Vec<_> = "foo".chars().enumerate().collect();
  println!("{:?}", enumerate);

  let zipper: Vec<_> = (0..).zip("foo".chars()).collect();
  println!("{:?}", zipper);
}
