fn main() {
    let n = "1";
    assert_eq!(n.parse::<i32>(), Ok(1));
    let n = "a";
    println!("{:?}", n.parse::<i32>());
}
