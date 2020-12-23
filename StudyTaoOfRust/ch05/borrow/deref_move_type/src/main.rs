fn join(s: &String) -> String {
    let append = &*s;
    "Hello".to_string() + append
}

fn main() {
    let x = "hello".to_string();
    join(&x);
    println!("main x = {}", x);
}
