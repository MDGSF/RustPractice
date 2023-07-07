fn main() {
    let mut x = 10;
    loop {
        x = if x % 2 == 0 { x / 2 } else { 3 * x + 1 };
        if x == 1 {
            break;
        }
    }
    println!("Final x: {x}");
}
