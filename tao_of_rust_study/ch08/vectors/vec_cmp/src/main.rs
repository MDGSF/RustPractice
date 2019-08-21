use std::cmp::Ordering;
fn main() {
    let result = 1.0.partial_cmp(&2.0);
    assert_eq!(result, Some(Ordering::Less));

    let result = 1.partial_cmp(&1);
    assert_eq!(result, Some(Ordering::Equal));

    let result = "abc".partial_cmp(&"Abc");
    assert_eq!(result, Some(Ordering::Greater));

    let mut v: [f32; 5] = [5.0, 4.1, 1.2, 3.4, 2.5];
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert!(v == [1.2, 2.5, 3.4, 4.1, 5.0]);

    v.sort_by(|a, b| b.partial_cmp(a).unwrap());
    assert!(v == [5.0, 4.1, 3.4, 2.5, 1.2]);
}
