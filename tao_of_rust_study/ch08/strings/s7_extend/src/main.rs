fn main() {
    let mut message = String::from("hello");
    message.extend([',', 'r', 'u'].iter());
    message.extend("st ".chars());
    message.extend("w o r l d".split_whitespace());
    println!("message = {:?}", message);
}
