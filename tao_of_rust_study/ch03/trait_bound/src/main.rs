use std::ops::Add;

fn sum<T: Add<T, Output = T>>(a: T, b: T) -> T {
    a + b
}

fn main() {
    println!("sum(1, 2) = {}", sum(1u32, 2u32));
}
