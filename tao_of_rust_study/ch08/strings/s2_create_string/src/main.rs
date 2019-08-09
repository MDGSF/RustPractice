fn main() {
    let string: String = String::new();
    assert_eq!("", string);

    let string: String = String::from("hello rust");
    assert_eq!("hello rust", string);

    let string: String = String::with_capacity(20);
    assert_eq!("", string);

    let str: &'static str = "the tao of rust";
    let string: String = str.chars().filter(|c| !c.is_whitespace()).collect();
    assert_eq!("thetaoofrust", string);

    let string: String = str.to_owned();
    assert_eq!("the tao of rust", string);

    let string: String = str.to_string();
    let str: &str = &string[11..15];
    assert_eq!("rust", str);
}
