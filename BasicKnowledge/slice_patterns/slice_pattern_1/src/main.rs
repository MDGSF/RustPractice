fn print_words(sentence: &str) {
  let words: Vec<_> = sentence.split_whitespace().collect();

  match words.as_slice() {
    [] => println!("There were no words"),
    [word] => println!("Found one word: {}", word),
    _ => println!("Found {} words: {:?}", words.len(), words),
  }
}

fn main() {
  print_words("");
  print_words("Hello");
  print_words("Hello World!");
}
