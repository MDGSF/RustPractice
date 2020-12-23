use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
  let input = input.join("");
  if input.len() == 0 {
    return HashMap::new();
  }

  let str_frequency = |input: &str| -> HashMap<char, usize> {
    let mut result: HashMap<char, usize> = HashMap::new();
    for ch in input
      .chars()
      .filter(|ch| ch.is_alphabetic())
      .map(|ch| ch.to_ascii_lowercase())
    {
      *result.entry(ch).or_default() += 1;
    }
    result
  };

  let mut churn = input.chars();
  let real_worker_count = worker_count.min(input.len());
  let mut thread_pool = Vec::with_capacity(real_worker_count);
  let work_length = (input.len() / real_worker_count).max(1);

  (1..=real_worker_count).for_each(|i| {
    let chunk = if i == real_worker_count {
      churn.by_ref().collect::<String>()
    } else {
      churn.by_ref().take(work_length).collect::<String>()
    };
    thread_pool.push(thread::spawn(move || str_frequency(&chunk)));
  });

  let mut result: HashMap<char, usize> = HashMap::new();
  for my_thread in thread_pool {
    let sub = my_thread.join().unwrap();
    for (&ch, num) in sub.iter() {
      *result.entry(ch).or_default() += num;
    }
  }
  result
}
