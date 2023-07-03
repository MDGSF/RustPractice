fn main() {
    let triple = [0, -2, 3];
    println!("Tell me about {triple:?}");
    match triple {
        [0, y, z] => println!("First is 0, y = {y}, and z = {z}"),
        [1, ..] => println!("Fist is 1 and the rest were ignored"),
        _ => println!("All elements were ignored"),
    }
}
