fn main() {
  let vector1 = vec![1, 2, 3];
  let vector1_a = vector1.iter().map(|x| x + 1).collect::<Vec<i32>>();
  let vector1_b = vector1.into_iter().map(|x| x * 10).collect::<Vec<i32>>();

  let mut vector2 = vec![10, 20, 30];
  vector2.iter_mut().for_each(|x| *x += 100);

  println!("{:?}", vector1_a);
  println!("{:?}", vector1_b);
  println!("{:?}", vector2);
}
