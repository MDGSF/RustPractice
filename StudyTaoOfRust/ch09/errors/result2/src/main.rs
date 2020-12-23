use std::num::ParseIntError;

type ParseResult<T> = Result<T, ParseIntError>;
fn square(number_str: &str) -> ParseResult<i32> {
    number_str.parse::<i32>().map(|n| n.pow(2))
}

fn main() {
    match square("10") {
        Ok(n) => assert_eq!(n, 100),
        Err(err) => println!("Error: {:?}", err),
    }
}
