/// (1 + 2 + ... + 10)² = 55² = 3025
/// S(n) = n*a1 + n*(n-1)*d/2
pub fn square_of_sum(n: u32) -> u32 {
  let sn = n * 1 + n * (n - 1) * 1 / 2;
  sn * sn
}

pub fn square_of_sum_2(n: u32) -> u32 {
  ((n * (n + 1)) / 2).pow(2)
}

pub fn square_of_sum_3(n: u32) -> u32 {
  (1..=n).sum::<u32>().pow(2)
}

/// 1² + 2² + ... + 10² = 385
/// 1²+2²+3²+4²+.....+n² =n(n+1)(2n+1)/6
pub fn sum_of_squares(n: u32) -> u32 {
  n * (n + 1) * (2 * n + 1) / 6
}

pub fn sum_of_squares_2(n: u32) -> u32 {
  (1..=n).map(|i| i.pow(2)).sum()
}

pub fn difference(n: u32) -> u32 {
  square_of_sum(n) - sum_of_squares(n)
}
