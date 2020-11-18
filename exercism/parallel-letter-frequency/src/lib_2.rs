use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
  (0..worker_count)
    .map(|i| {
      let mut result = HashMap::new();
      input
        .join("")
        .chars()
        .skip(i)
        .step_by(worker_count)
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .for_each(|c| {
          *result.entry(c).or_insert(0) += 1;
        });
      result
    })
    .fold(HashMap::new(), |mut result, m| {
      m.iter().for_each(|(&k, &v)| {
        *result.entry(k).or_insert(0) += v;
      });
      result
    })
}
