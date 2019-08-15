use std::collections::HashMap;

fn main() {
    let mut book_reviews = HashMap::with_capacity(10);
    book_reviews.insert("Rust Book", "good");
    book_reviews.insert("Programming Rust", "nice");
    book_reviews.insert("The Tao of Rust", "deep");

    for key in book_reviews.keys() {
        println!("{}", key);
    }

    for val in book_reviews.values() {
        println!("{}", val);
    }

    if !book_reviews.contains_key("rust book") {
        println!("find {} times", book_reviews.len());
    }

    book_reviews.remove("Rust Book");

    let to_find = ["Rust Book", "The Tao of Rust"];
    for book in &to_find {
        match book_reviews.get(book) {
            Some(review) => println!("{}: {}", book, review),
            None => println!("{} is unreviewed", book),
        }
    }

    for (book, review) in &book_reviews {
        println!("{}: \"{}\"", book, review);
    }

    assert_eq!(book_reviews["The Tao of Rust"], "deep");
}
