#[derive(Debug, Clone)]
struct Library {
  library_type: LibraryType,
  books: Vec<String>,
}

#[derive(Debug, Clone)]
enum LibraryType {
  City,
  Country,
}

impl Library {
  fn new() -> Self {
    Self {
      library_type: LibraryType::City,
      books: Vec::new(),
    }
  }

  fn add_book(&mut self, book: &str) {
    self.books.push(book.to_string());
  }
}

impl Iterator for Library {
  type Item = String;

  fn next(&mut self) -> Option<String> {
    match self.books.pop() {
      Some(book) => Some(book + " is found!"),
      None => None,
    }
  }
}

fn main() {
  let mut my_library = Library::new();
  my_library.add_book("The Doom of Darksword");
  my_library.add_book("Harry Potter");

  for item in my_library.clone() {
    println!("{}", item);
  }
}
