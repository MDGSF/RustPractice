use std::boxed::Box;

fn main() {
    println!("{}", std::mem::size_of::<&[u32; 5]>());
    println!("{}", std::mem::size_of::<&[u32; 10]>());
    println!("{}", std::mem::size_of::<i32>());
}
