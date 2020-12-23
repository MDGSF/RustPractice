struct A {
    a: u32,
    b: Box<u64>,
}

struct B(i32, f64, char);

struct N;

enum E {
    H(u32),
    M(Box<u32>),
}

union U {
    u: u32,
    v: u64,
}

fn main() {
    println!("char = {}", std::mem::size_of::<char>()); // 4
    println!("i32 = {}", std::mem::size_of::<i32>()); // 4
    println!("u32 = {}", std::mem::size_of::<u32>()); // 4
    println!("u64 = {}", std::mem::size_of::<u64>()); // 8
    println!("f64 = {}", std::mem::size_of::<f64>()); // 8
    println!("Box<u32> = {}", std::mem::size_of::<Box<u32>>()); // 8
    println!("Box<u64> = {}", std::mem::size_of::<Box<u64>>()); // 8

    println!("A = {}", std::mem::size_of::<A>()); // 16
    println!("B = {}", std::mem::size_of::<B>()); // 16
    println!("N = {}", std::mem::size_of::<N>()); // 0
    println!("E = {}", std::mem::size_of::<E>()); // 16
    println!("U = {}", std::mem::size_of::<U>()); // 8
}
