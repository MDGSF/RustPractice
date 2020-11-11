/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
  code
    .chars()
    .rev()
    .filter(|c| !c.is_whitespace())
    .try_fold((0, 0), |(sum, count), c| {
      c.to_digit(10)
        .map(|num| if count % 2 == 1 { num * 2 } else { num })
        .map(|num| if num > 9 { num - 9 } else { num })
        .map(|num| (sum + num, count + 1))
    })
    .map_or(false, |(sum, count)| sum % 10 == 0 && count > 1)
}
