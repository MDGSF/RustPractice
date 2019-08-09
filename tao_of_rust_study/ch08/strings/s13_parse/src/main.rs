fn main() {
    let four: u32 = "4".parse().unwrap();
    assert_eq!(4, four);

    let four = "4".parse::<u32>();
    assert_eq!(Ok(4), four);
}
