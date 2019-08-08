fn main() {
    let a = [1, 2, 3];
    let mut iter = a.iter();
    assert_eq!(iter.nth(1), Some(&2));
    println!("{:?}", iter.nth(0));
    println!("{:?}", iter.nth(0));

    assert_eq!(iter.nth(1), None);
}
