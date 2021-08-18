fn main() {
  println!("Hello, world!");
}

fn word_is_palindrome(word: &str) -> bool {
  let letters: Vec<_> = word.chars().collect();
  is_palindrome(&letters)
}

fn is_palindrome(items: &[char]) -> bool {
  match items {
    [first, middle @ .., last] => first == last && is_palindrome(middle),
    [] | [..] => true,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn known_palindromes() {
    assert!(word_is_palindrome(""));
    assert!(word_is_palindrome("a"));
    assert!(word_is_palindrome("aba"));
    assert!(word_is_palindrome("abba"));
  }

  #[test]
  fn not_palindromes() {
    assert!(!word_is_palindrome("abc"));
    assert!(!word_is_palindrome("abab"));
  }
}
