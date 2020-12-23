fn main() {
    enum Void {}
    struct Foo;
    struct Baz {
        foo: Foo,
        qux: (),
        baz: [u8; 0],
    }
    assert_eq!(std::mem::size_of::<()>(), 0);
    assert_eq!(std::mem::size_of::<Void>(), 0);
    assert_eq!(std::mem::size_of::<Foo>(), 0);
    assert_eq!(std::mem::size_of::<Baz>(), 0);
    assert_eq!(std::mem::size_of::<[(); 10]>(), 0);
}
