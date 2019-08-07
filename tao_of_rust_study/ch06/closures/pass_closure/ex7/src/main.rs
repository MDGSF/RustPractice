fn square() -> impl Fn(i32) -> i32 {
    |i| i * i
}

fn main() {
    let s = square();
    assert_eq!(4, s(2));
    assert_eq!(9, s(3));
}
