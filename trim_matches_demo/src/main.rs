fn main() {
    println!("Hello, world!");

    assert_eq!("11foo1bar11".trim_matches('1'), "foo1bar");
}
