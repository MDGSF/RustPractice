fn main() {
    let a = [1, 2, 3];
    assert_eq!(a.iter().any(|&x| x != 2), true);
    let sum = a.iter().fold(0, |acc, x| acc + x);
    assert_eq!(sum, 6);
}
