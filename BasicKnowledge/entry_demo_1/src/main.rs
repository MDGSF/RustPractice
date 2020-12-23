use std::collections::HashMap;

fn main() {
  let book_collections = vec![
    "Allemagne Moderne",
    "Le Petit Prince",
    "Eye of the World",
    "Eye of the World",
  ];

  let mut book_hashmap = HashMap::new();

  for book in book_collections {
    let return_value = book_hashmap.entry(book).or_insert(0);
    *return_value += 1;
  }

  for (book, number) in book_hashmap {
    println!("{}, {}", book, number);
  }
}
