fn square() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|i| i * i)
}

fn square2() -> Box<dyn FnOnce(i32) -> i32> {
    Box::new(|i| i * i)
}

fn main() {
    let square = square();
    assert_eq!(4, square(2));
    assert_eq!(9, square(3));

    let square2 = square2();
    assert_eq!(4, square2(2));
    //assert_eq!(4, square2(2));
}
