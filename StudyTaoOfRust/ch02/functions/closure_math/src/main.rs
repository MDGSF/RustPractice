pub fn math<F: Fn() -> i32>(op: F) -> i32 {
    op()
}

fn main() {
    let a = 2;
    let b = 3;
    assert_eq!(math(|| a + b), 5);
    assert_eq!(math(|| a * b), 6);
}
