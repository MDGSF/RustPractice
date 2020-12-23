fn two_times() -> Box<dyn Fn(i32) -> i32> {
    let i = 2;
    Box::new(move |j| j * i)
}

fn main() {
    let result = two_times();
    assert_eq!(result(2), 4);
}
