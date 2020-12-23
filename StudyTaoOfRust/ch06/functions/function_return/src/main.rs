fn addsub(x: isize, y: isize) -> (isize, isize) {
    (x + y, x - y)
}

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        return a;
    }
    return gcd(b, a % b);
}

fn main() {
    let (a, b) = addsub(5, 8);
    println!("a: {:?}, b: {:?}", a, b);

    let g = gcd(60, 40);
    assert_eq!(20, g);
}
