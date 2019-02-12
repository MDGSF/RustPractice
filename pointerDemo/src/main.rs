fn main() {
    println!("Hello, world!");
    let mut num = 5;
    let r1 = &mut num as *mut i32;
    let r2 = r1 as u64;

    println!("r1 = {:?}, r2 = {:?}", r1, r2);
}
