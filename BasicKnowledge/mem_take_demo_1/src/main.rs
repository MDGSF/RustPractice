use std::mem;

fn main() {
  let mut number_vec = vec![8, 7, 0, 2, 49, 9999];
  let mut new_vec = vec![];

  number_vec.iter_mut().for_each(|number| {
    let taker = mem::take(number);
    new_vec.push(taker);
  });

  println!("{:?}\n{:?}", number_vec, new_vec);
}
