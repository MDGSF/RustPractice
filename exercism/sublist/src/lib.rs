#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
  Equal,
  Sublist,
  Superlist,
  Unequal,
}

pub fn sublist<T>(first_list: &[T], second_list: &[T]) -> Comparison
where
  T: PartialEq,
{
  use Comparison::*;
  match (first_list.len(), second_list.len()) {
    (0, 0) => Equal,
    (0, _) => Sublist,
    (_, 0) => Superlist,
    (m, n) if m > n => {
      if first_list
        .windows(n)
        .any(|candidate| candidate == second_list)
      {
        Superlist
      } else {
        Unequal
      }
    }
    (m, n) if m < n => {
      if second_list
        .windows(m)
        .any(|candidate| candidate == first_list)
      {
        Sublist
      } else {
        Unequal
      }
    }
    (_, _) => {
      if first_list == second_list {
        Equal
      } else {
        Unequal
      }
    }
  }
}
