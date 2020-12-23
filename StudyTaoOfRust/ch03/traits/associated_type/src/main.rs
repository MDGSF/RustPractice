#[derive(Debug, PartialEq)]
struct Foo(i32);

#[derive(Debug, PartialEq)]
struct Bar(i32, i32);

trait Inst {
    type F;
    type B;
    fn new_foo(i: i32) -> Self::F;
    fn new_bar(i: i32) -> Self::B;
}

struct FooBar;

impl Inst for FooBar {
    type F = Foo;
    type B = Bar;
    fn new_foo(i: i32) -> Foo {
        Foo(i)
    }
    fn new_bar(i: i32) -> Bar {
        Bar(i, i + 10)
    }
}

fn main() {
    let f: Foo = FooBar::new_foo(10);
    assert_eq!(f, Foo(10));
    let b: Bar = FooBar::new_bar(20);
    assert_eq!(b, Bar(20, 30));
}
