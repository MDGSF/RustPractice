#[derive(Debug, Copy, Clone)]
struct Book<'a> {
    name: &'a str,
    isbn: i32,
    version: i32,
}

fn main() {
    let book = Book {
        name: "tao of rust",
        isbn: 12345,
        version: 1,
    };

    let book2 = Book { version: 2, ..book };

    println!("book = {:?}", book);
    println!("book2 = {:?}", book2);
}
