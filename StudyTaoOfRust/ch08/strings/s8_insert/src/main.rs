fn main() {
    let mut s = String::with_capacity(3);
    s.insert(0, 'f');
    s.insert(1, 'o');
    s.insert(2, 'o');
    s.insert_str(0, "bar");
    assert_eq!("barfoo", s);
}
