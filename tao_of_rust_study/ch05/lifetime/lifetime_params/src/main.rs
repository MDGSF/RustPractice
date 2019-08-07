fn the_longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s1 = String::from("Rust");
    let s2 = String::from("C");
    let result = the_longest(&s1, &s2);
    println!("result = {}", result);
}
