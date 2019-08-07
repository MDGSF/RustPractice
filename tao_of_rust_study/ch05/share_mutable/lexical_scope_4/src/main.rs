fn main() {
    let a = Some("hello".to_string());
    if let Some(s) = a {
        println!("{:?}", s);
    }
    println!("{:?}", a);
}
