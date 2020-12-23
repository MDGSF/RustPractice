fn main() {
    let arr = vec![1, 2, 3];
    let sum1 = arr.iter().fold(0, |acc, x| acc + x);
    let sum2 = arr.iter().fold(0, |acc, x| acc + *x);
    let sum3 = arr.iter().fold(0, |acc, &x| acc + x);
    let sum4 = arr.into_iter().fold(0, |acc, x| acc + x);
    assert_eq!(sum1, 6);
    assert_eq!(sum2, 6);
    assert_eq!(sum3, 6);
    assert_eq!(sum4, 6);
}
