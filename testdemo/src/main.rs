use std::collections::HashMap;

fn main() {
    let mut m = HashMap::new();
    m.insert("aa", "1");
    m.insert("aa", 2);
    println!("m = {:?}", m);
}
