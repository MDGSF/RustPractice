pub fn square(s: u32) -> u64 {
  if s <= 0 || s > 64 {
    panic!("Square must be between 1 and 64");
  }
  2_u64.pow(s - 1)
}

/// s = 2 ^ (n + 1) - 1
/// 2 ^ 0 = 1
/// 2 ^ 1 = 2
/// 2 ^ 2 = 4
pub fn total() -> u64 {
  (2_u128.pow(64 + 1) - 1) as u64
}
