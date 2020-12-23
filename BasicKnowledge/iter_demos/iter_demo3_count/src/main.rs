fn main() {
  let a = [1, 2, 3];
  assert_eq!(a.iter().count(), 3);

  let a = [1, 2, 3, 4, 5];
  let mut iter = a.iter();
  println!("{:?}", iter.next());
  assert_eq!(iter.count(), 4);
  assert_eq!(a.iter().count(), 5);
}
