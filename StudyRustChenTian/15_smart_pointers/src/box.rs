fn main() {
    let boxed = Box::new([0u8; 1 << 24]);
    println!("len: {}", boxed.len());
}
