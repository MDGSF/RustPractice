fn main() {
    let mut v = String::from("borös");
    assert_eq!(Some("b"), v.get(0..1));
    assert_eq!(Some("ö"), v.get(3..5));
    assert_eq!(Some("orös"), v.get(1..));
    assert!(v.get_mut(4..).is_none());
    assert!(!v.is_char_boundary(4));
    assert!(v.get_mut(..8).is_none());
    assert!(v.get_mut(..42).is_none());
}
