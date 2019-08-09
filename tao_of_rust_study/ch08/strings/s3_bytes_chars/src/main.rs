fn main() {
    let str = "borös";
    let mut chars = str.chars();
    assert_eq!(Some('b'), chars.next());
    assert_eq!(Some('o'), chars.next());
    assert_eq!(Some('r'), chars.next());
    assert_eq!(Some('ö'), chars.next());
    assert_eq!(Some('s'), chars.next());

    let mut bytes = str.bytes();
    assert_eq!(6, str.len());
    assert_eq!(Some(98), bytes.next());
}
