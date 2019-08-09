fn main() {
    let left = "the tao".to_string();
    let mut right = "Rust".to_string();
    assert_eq!(left + " of " + &right, "the tao of Rust");
    right += "!";
    assert_eq!(right, "Rust!");
}
