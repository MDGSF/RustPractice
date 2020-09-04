use std::convert::From;

struct EvenOddVec(Vec<Vec<i32>>);

impl From<Vec<i32>> for EvenOddVec {
  fn from(input: Vec<i32>) -> Self {
    let mut even_odd_vec = vec![vec![], vec![]];

    for item in input {
      if item % 2 == 0 {
        even_odd_vec[0].push(item);
      } else {
        even_odd_vec[1].push(item);
      }
    }

    Self(even_odd_vec)
  }
}

fn main() {
  let bunch_of_numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
  let new_vec = EvenOddVec::from(bunch_of_numbers);
  println!("Even numbers: {:?}", new_vec.0[0]);
  println!("Odd numbers: {:?}", new_vec.0[1]);
}
