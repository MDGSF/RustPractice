fn call<F>(closure: F) -> i32
where
    F: Fn(i32) -> i32,
{
    closure(1)
}

fn counter(i: i32) -> i32 {
    i + 1
}

fn main() {
    let result = call(counter);
    assert_eq!(2, result);
}
