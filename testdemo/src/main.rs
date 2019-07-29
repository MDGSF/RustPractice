fn main() {
    let s = String::from("test");
    let ptr = s.as_ptr();
    println!("ptr = {:p}", ptr);
    println!("&s = {:p}", &s);
}
