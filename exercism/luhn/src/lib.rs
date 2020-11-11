/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
  code
    .chars()
    .rev()
    .filter(|&c| c != ' ')
    .map(|c| c.to_digit(10))
    .enumerate()
    .map(|(i, digit)| {
      digit.map(|digit| {
        if i % 2 == 1 {
          let n = digit * 2;
          if n > 9 {
            n - 9
          } else {
            n
          }
        } else {
          digit
        }
      })
    })
    .try_fold((0, 0), |(count, sum), digit| {
      digit.map(|digit| (count + 1, sum + digit))
    })
    .map_or(false, |(count, sum)| count > 1 && sum % 10 == 0)
}
