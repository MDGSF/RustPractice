fn main() {
    let mut a = String::from("food");
    println!("{:p}", a.as_ptr());
    println!("{:p}", &a);
    assert_eq!(a.len(), 4);
    a.reserve(10);
    assert!(a.capacity() >= 10);
}
