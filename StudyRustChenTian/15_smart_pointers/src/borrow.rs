use std::borrow::Borrow;

fn main() {
    let s = "hello world!".to_owned();

    let r1: &String = s.borrow();

    let r2: &str = s.borrow();

    println!("r1: {:p}, r2: {:p}", r1, r2);
}
