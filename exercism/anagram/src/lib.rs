use std::collections::HashMap;
use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
  let word = word.to_lowercase();
  let mut sword: HashMap<char, u32> = HashMap::new();
  for c in word.chars() {
    *sword.entry(c).or_insert(0) += 1;
  }

  let mut result = HashSet::new();
  for &candidate in possible_anagrams.iter() {
    let low_candidate = candidate.to_lowercase();
    if word.len() != low_candidate.len() || word == *low_candidate {
      continue;
    }

    let mut sw = HashMap::new();
    for c in low_candidate.chars() {
      *sw.entry(c.to_ascii_lowercase()).or_insert(0) += 1;
    }
    if sword == sw {
      result.insert(candidate);
    }
  }
  result
}
