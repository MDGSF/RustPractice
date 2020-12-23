fn main() {
    let s = "Per Martin-Löf";
    let (first, last) = s.split_at(12);
    println!("first = {:?}", first);
    println!("last = {:?}", last);
}
