fn main() {
    let n = 13;
    let big_n = if n < 10 && n > -10 { 10 * n } else { n / 2 };
    println!("big_n = {}", big_n);
}
