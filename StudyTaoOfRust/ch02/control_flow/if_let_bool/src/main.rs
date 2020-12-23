fn main() {
    let boolean = true;
    let mut binary = 0;

    if let true = boolean {
        binary = 1;
    }
    println!("binary = {}", binary);

    if boolean {
        binary = 2;
    }
    println!("binary = {}", binary);

    let number = Some(7);
    if let Some(i) = number {
        println!("Matched = {:?}", i);
    }
}
