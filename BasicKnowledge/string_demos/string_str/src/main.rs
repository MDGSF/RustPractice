/*
https://www.cs.brandeis.edu/~cs146a/rust/doc-02-21-2015/book/strings.html
https://www.cs.brandeis.edu/~cs146a/rust/doc-02-21-2015/book/more-strings.html

String and &str

String, is a heap-allocated string, is growable, is guaranteed to be UTF-8.

&str, is called string slices.
Like vector slices, string slices are simply a pointer plus a length.
This means that they're a 'view' into an already-allocated string, such as a string literal or a String.

Viewing a String as a &str is cheap, but converting the &str to a String involves allocating memory.
*/

fn main() {
    test1();
    test2();
}

/*
建议使用：在拥有所有权的地方使用 String，在租借出去的时候使用 &str
*/
fn test1() {
    let s1 = "s1 hello world"; // s1: &str

    let mut s2 = "s2 Hello".to_string();
    s2.push_str(", world.");

    take_slice(s1);
    take_slice(&s2);
}

fn take_slice(slice: &str) {
    println!("Got: {}", slice);
}

/*
.as_bytes() convert to &[u8].
.bytes() will iterate over the underlying bytes
.chars() will iterate over the code points
*/
fn test2() {
    let s = "hello world.";
    for c in s.bytes() {
        print!("{} ", c);
    }
    println!();
    for c in s.chars() {
        print!("{} ", c);
    }
    println!();
}
