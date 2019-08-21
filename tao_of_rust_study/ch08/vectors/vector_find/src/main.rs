fn main() {
    let v = [10, 40, 30];
    assert!(v.contains(&30));
    assert!(!v.contains(&50));
    assert!(v.starts_with(&[10]));
    assert!(v.starts_with(&[10, 40]));
    assert!(v.ends_with(&[30]));
    assert!(v.ends_with(&[40, 30]));
    assert!(v.ends_with(&[]));

    let v: &[u8] = &[];
    assert!(v.starts_with(&[]));
    assert!(v.ends_with(&[]));
}
