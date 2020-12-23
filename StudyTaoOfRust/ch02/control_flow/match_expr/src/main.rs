fn main() {
    let number = 42;
    match number {
        0 => println!("Origin"),
        1..=3 => println!("All"),
        5 | 7 | 13 => println!("Bad Luck"),
        n @ 42 => println!("Answer is {}", n),
        _ => println!("Common"),
    }
}
