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
  if first_list == second_list {
    Comparison::Equal
  } else if first_list.len() == 0
    || second_list
      .windows(first_list.len())
      .any(|v| v == first_list)
  {
    Comparison::Sublist
  } else if second_list.len() == 0
    || first_list
      .windows(second_list.len())
      .any(|v| v == second_list)
  {
    Comparison::Superlist
  } else {
    Comparison::Unequal
  }
}
