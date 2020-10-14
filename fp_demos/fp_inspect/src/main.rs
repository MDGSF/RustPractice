fn main() {
  let new_vec = vec![8, 9, 10];

  let double_vec = new_vec
    .iter()
    .inspect(|item| println!("Before item is: {}", item))
    .map(|x| x * 2)
    .inspect(|item| println!("After item is: {}", item))
    .collect::<Vec<i32>>();

  println!("{:?}", double_vec);
}
