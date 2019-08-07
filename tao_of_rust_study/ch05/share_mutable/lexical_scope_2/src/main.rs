fn main() {
    let a = Some("hello".to_string());
    match a {
        Some(s) => println!("{:?}", s),
        _ => println!("nothing"),
    }
    //println!("{:?}", a);
}
