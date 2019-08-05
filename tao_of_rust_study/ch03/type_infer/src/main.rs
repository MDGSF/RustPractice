fn sum(a: u32, b: i32) -> u32 {
    a + (b as u32)
}

fn main() {
    let a = 1;
    let b = 2;
    assert_eq!(sum(a, b), 3);

    let elem = 5u8;
    let mut vec = Vec::new();
    vec.push(elem);
    assert_eq!(vec, [5]);
}
