#[derive(Debug)]
struct S {
    i: i32,
}

fn f(ref _s: S) {
    println!("{:p}", _s);
}

fn foo(_: i32) {}

fn swap((x, y): (&str, i32)) -> (i32, &str) {
    (y, x)
}

fn main() {
    let s = S { i: 42 };
    f(s);

    foo(3);

    let t = ("Alex", 18);
    let t = swap(t);
    assert_eq!(t, (18, "Alex"));
}
