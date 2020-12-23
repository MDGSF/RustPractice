fn counter(i: i32) -> impl Fn(i32) -> i32 {
    move |n: i32| n + i
}

fn main() {
    let f = counter(3);
    assert_eq!(4, f(1));
}
