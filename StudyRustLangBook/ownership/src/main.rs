fn main() {
    let s1 = String::from("Hello");
    let (s2, len) = calculate_length(s1);
    println!("{}, {}", s2, len);

    let len = calculate_length_borrow(&s2);
    println!("len = {}", len);

    test_mut();
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_borrow(s: &String) -> usize {
    return s.len();
}

fn test_mut() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("s = {}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
