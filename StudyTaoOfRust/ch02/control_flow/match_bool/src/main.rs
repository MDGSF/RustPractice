fn main() {
    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };
    println!("binary = {}", binary);
}
