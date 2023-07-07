fn main() {
    let numbers = vec![10, 20, 30];
    let first: Option<&i32> = numbers.first();
    println!("first: {first:?}");

    let idx: Result<usize, usize> = numbers.binary_search(&10);
    println!("idx: {idx:?}");
}
