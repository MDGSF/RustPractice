fn main() {
    let mut s = String::from("hαllo");
    assert_eq!(6, s.len());

    s.remove(3);
    assert_eq!("hαlo", s);
    assert_eq!(Some('o'), s.pop());
    assert_eq!(Some('l'), s.pop());
    assert_eq!(Some('α'), s.pop());
    assert_eq!("h", s);

    let mut s = String::from("hαllo");

    s.truncate(3);
    assert_eq!("hα", s);

    s.clear();
    assert_eq!("", s);

    let mut s = String::from("α is alpha, β is beta");
    let beta_offset = s.find('β').unwrap_or(s.len());
    let t: String = s.drain(..beta_offset).collect();
    assert_eq!("α is alpha, ", t);
    assert_eq!("β is beta", s);
    s.drain(..);
    assert_eq!("", s);
}
