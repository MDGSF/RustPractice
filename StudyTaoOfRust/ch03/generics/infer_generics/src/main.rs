#[derive(Debug, PartialEq)]
struct Foo(i32);

#[derive(Debug, PartialEq)]
struct Bar(i32, i32);

trait Inst {
    fn new(i: i32) -> Self;
}

impl Inst for Foo {
    fn new(i: i32) -> Foo {
        Foo(i)
    }
}

impl Inst for Bar {
    fn new(i: i32) -> Bar {
        Bar(i, i + 10)
    }
}

fn foobar<T: Inst>(i: i32) -> T {
    T::new(i)
}

fn main() {
    let f: Foo = foobar(10);
    assert_eq!(f, Foo(10));

    let b: Bar = foobar(20);
    assert_eq!(b, Bar(20, 30));
}
