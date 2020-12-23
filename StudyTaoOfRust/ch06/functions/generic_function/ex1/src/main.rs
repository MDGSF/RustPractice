use std::ops::Mul;

fn square<T: Mul<T, Output = T>>(x: T, y: T) -> T {
    x * y
}

fn main() {
    let a: i32 = square(37, 41);
    let b: f64 = square(37.2, 41.1);
    println!("a = {}", a);
    println!("b = {}", b);
}
