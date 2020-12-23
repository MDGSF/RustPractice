// https://doc.rust-lang.org/std/collections/struct.HashMap.html

use std::collections::HashMap;

fn main() {
  let mut book_reviews = HashMap::new();

  book_reviews.insert(
    "Adventures of Huckleberry Finn".to_string(),
    "My favorite book.".to_string(),
  );

  book_reviews.insert(
    "Grimms' Fairy Tales".to_string(),
    "Masterpiece.".to_string(),
  );

  book_reviews.insert(
    "Pride and Prejudice".to_string(),
    "Very enjoyable.".to_string(),
  );

  book_reviews.insert(
    "The Adventures of Sherlock Holmes".to_string(),
    "Eye lyked it alot".to_string(),
  );

  if !book_reviews.contains_key("Les Misérables") {
    println!(
      "We've got {} reviews, but Les Misérables ain't one.",
      book_reviews.len()
    );
  }

  book_reviews.remove("The Adventures of Sherlock Holmes");

  let to_find = ["Pride and Prejudice", "Alice's Adventure in Wonderland"];
  for &book in &to_find {
    match book_reviews.get(book) {
      Some(review) => println!("{}: {}", book, review),
      None => println!("{} is unreviewed", book),
    }
  }

  println!("Review for Jane: {}", book_reviews["Pride and Prejudice"]);

  for (book, review) in &book_reviews {
    println!("{}: \"{}\"", book, review);
  }
}
