/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
  if code.len() <= 1 {
    return false;
  }

  if code.chars().any(|c| !c.is_digit(10) && !c.is_whitespace()) {
    return false;
  }

  if code.chars().filter(|c| c.is_digit(10)).count() <= 1 {
    return false;
  }

  code
    .chars()
    .rev()
    .filter(|c| c.is_digit(10))
    .enumerate()
    .fold(0, |sum, (i, c)| {
      let val = if i % 2 == 1 {
        let dc = c.to_digit(10).unwrap() * 2;
        if dc > 9 {
          dc - 9
        } else {
          dc
        }
      } else {
        c.to_digit(10).unwrap()
      };
      sum + val
    })
    % 10
    == 0
}
