fn main() {
    let s = "hello";
    let c = move || println!("{:?}", s);
    c();
    c();
    println!("{:?}", s);
}
