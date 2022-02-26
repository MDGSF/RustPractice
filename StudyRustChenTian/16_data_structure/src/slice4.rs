use std::fmt;

fn main() {
    let s = String::from("hello");
    print_slice(&s);
    print_slice(&s[..]);

    print_slice1(&s);
    print_slice1(&s[..]);
    print_slice1(s.clone());

    print_slice2(&s);
    print_slice2(&s[..]);
    print_slice2(s);
}

fn print_slice(s: &str) {
    println!("{:?}", s);
}

fn print_slice1<T: AsRef<str>>(s: T) {
    println!("{:?}", s.as_ref());
}

fn print_slice2<T, U>(s: T)
where
    T: AsRef<[U]>,
    U: fmt::Debug,
{
    println!("{:?}", s.as_ref());
}

#[allow(dead_code)]
fn print_slice3<T, U>(s: T)
where
    T: AsRef<U>,
    U: fmt::Debug,
{
    println!("{:?}", s.as_ref());
}
