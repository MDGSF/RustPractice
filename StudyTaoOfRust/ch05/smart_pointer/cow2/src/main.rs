use std::borrow::Cow;
use std::thread;

#[derive(Debug)]
struct Token<'a> {
    raw: Cow<'a, str>,
}

impl<'a> Token<'a> {
    pub fn new<S>(raw: S) -> Token<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Token { raw: raw.into() }
    }
}

fn main() {
    let token = Token::new("abc123");
    thread::spawn(move || {
        println!("token: {:?}", token);
    })
    .join()
    .unwrap();
}
