enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn test1() {
    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
    println!("bird are #{:06x}", Color::Green as i32);
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn test2() {
    let home = IpAddr::V4(127, 0, 0, 1);
}

enum MyOption {
    Some(i32),
    None,
}

enum Number {
    Zero,
    One,
    Two,
}

fn main() {
    test1();
}
