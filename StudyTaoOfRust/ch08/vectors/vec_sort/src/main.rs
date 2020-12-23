fn main() {
    let mut v = [-5i32, 4, 1, -3, 2];

    v.sort();
    assert!(v == [-5, -3, 1, 2, 4]);

    v.sort_by(|a, b| a.cmp(b));
    assert!(v == [-5, -3, 1, 2, 4]);

    v.sort_by(|a, b| b.cmp(a));
    assert!(v == [4, 2, 1, -3, -5]);

    v.sort_by_key(|k| k.abs());
    assert!(v == [1, 2, -3, 4, -5]);
}
