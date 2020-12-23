extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
  UnicodeSegmentation::graphemes(input, true)
    .rev()
    .collect::<String>()
}

pub fn reverse_2(input: &str) -> String {
  input.chars().rev().collect::<String>()
}
