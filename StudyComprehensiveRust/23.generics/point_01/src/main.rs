#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let p = Point { x: 5, y: 1.0 };
    println!("{integer:?} and {float:?} and {p:?}");
}
