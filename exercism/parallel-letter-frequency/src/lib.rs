use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
  let input: Vec<String> = input.iter().map(|item| item.to_string()).collect();

  let str_frequency = |input: &str| -> HashMap<char, usize> {
    let mut result: HashMap<char, usize> = HashMap::new();
    for ch in input
      .chars()
      .filter(|ch| ch.is_alphabetic())
      .map(|ch| ch.to_ascii_lowercase())
    {
      let counter = result.entry(ch).or_insert(0);
      *counter += 1;
    }
    result
  };

  let input = Arc::new(input);
  let (sender, receiver) = channel();

  for start in 0..worker_count {
    let sub_input = Arc::clone(&input);
    let sub_sender = sender.clone();
    thread::spawn(move || {
      for i in (start..sub_input.len()).step_by(worker_count) {
        let sub = str_frequency(&sub_input[i]);
        sub_sender.send(sub).unwrap();
      }
    });
  }

  drop(sender);

  let mut result: HashMap<char, usize> = HashMap::new();
  for sub in receiver {
    for (ch, num) in sub {
      let counter = result.entry(ch).or_insert(0);
      *counter += num;
    }
  }
  result
}
