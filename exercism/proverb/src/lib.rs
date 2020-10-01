pub fn build_proverb(list: &[&str]) -> String {
  list
    .iter()
    .zip(list.iter().skip(1))
    .map(|(a, b)| format!("For want of a {} the {} was lost.", a, b))
    .chain(
      list
        .iter()
        .take(1)
        .map(|a| format!("And all for the want of a {}.", a)),
    )
    .collect::<Vec<_>>()
    .join("\n")
}

pub fn build_proverb_2(list: &[&str]) -> String {
  if list.is_empty() {
    return String::new();
  }

  list
    .windows(2)
    .map(|win| format!("For want of a {} the {} was lost.", win[0], win[1]))
    .chain(std::iter::once(format!(
      "And all for the want of a {}.",
      list[0]
    )))
    .collect::<Vec<_>>()
    .join("\n")
}

pub fn build_proverb_1(list: &[&str]) -> String {
  let mut result = String::new();
  for (index, word) in list.iter().enumerate() {
    if index < list.len() - 1 {
      result.push_str(&format!(
        "For want of a {} the {} was lost.\n",
        word,
        list[index + 1]
      ));
    } else {
      result.push_str(&format!("And all for the want of a {}.", list[0]));
    }
  }
  result
}
