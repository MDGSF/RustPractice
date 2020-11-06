#[derive(Debug, PartialEq)]
pub enum Comparison {
  Equal,
  Sublist,
  Superlist,
  Unequal,
}

pub fn sublist<T>(first_list: &[T], second_list: &[T]) -> Comparison
where
  T: PartialEq + Copy,
{
  if first_list.len() > second_list.len() {
    for k1 in 0..=(first_list.len() - second_list.len()) {
      let mut success = true;
      for (k2, &v2) in second_list.iter().enumerate() {
        if v2 != first_list[k1 + k2] {
          success = false;
          break;
        }
      }
      if success {
        return Comparison::Superlist;
      }
    }
  } else if first_list.len() < second_list.len() {
    for k2 in 0..=(second_list.len() - first_list.len()) {
      let mut success = true;
      for (k1, &v1) in first_list.iter().enumerate() {
        if v1 != second_list[k1 + k2] {
          success = false;
          break;
        }
      }
      if success {
        return Comparison::Sublist;
      }
    }
  } else {
    for (k, &v) in first_list.iter().enumerate() {
      if v != second_list[k] {
        return Comparison::Unequal;
      }
    }
    return Comparison::Equal;
  }

  Comparison::Unequal
}
