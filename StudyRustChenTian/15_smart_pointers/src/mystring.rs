use std::{fmt, ops::Deref, str};

const MINI_STRING_MAX_LEN: usize = 30;

struct MiniString {
    len: u8,
    data: [u8; MINI_STRING_MAX_LEN],
}

impl MiniString {
    fn new(v: impl AsRef<str>) -> Self {
        let bytes = v.as_ref().as_bytes();
        let len = bytes.len();
        let mut data = [0u8; MINI_STRING_MAX_LEN];
        data[..len].copy_from_slice(bytes);
        Self {
            len: len as u8,
            data,
        }
    }
}

impl Deref for MiniString {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        str::from_utf8(&self.data[..self.len as usize]).unwrap()
    }
}

impl fmt::Debug for MiniString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.deref())
    }
}

#[derive(Debug)]
enum MyString {
    Inline(MiniString),
    Standard(String),
}

impl Deref for MyString {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        match *self {
            MyString::Inline(ref v) => v.deref(),
            MyString::Standard(ref v) => v.deref(),
        }
    }
}

impl<T> From<T> for MyString
where
    T: AsRef<str>,
{
    fn from(s: T) -> Self {
        match s.as_ref().len() > MINI_STRING_MAX_LEN {
            true => Self::Standard(s.as_ref().to_owned()),
            _ => Self::Inline(MiniString::new(s)),
        }
    }
}

impl fmt::Display for MyString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.deref())
    }
}

impl MyString {
    pub fn push_str(&mut self, s: &str) {
        match *self {
            MyString::Inline(ref mut v) => {
                let len = v.len() as usize;
                let len1 = s.len();
                if len + len1 > MINI_STRING_MAX_LEN {
                    let mut owned = v.deref().to_string();
                    owned.push_str(s);
                    *self = MyString::Standard(owned);
                } else {
                    let total = len + len1;
                    v.data[len..len + len1].copy_from_slice(s.as_bytes());
                    v.len = total as u8;
                }
            }
            MyString::Standard(ref mut v) => v.push_str(s),
        }
    }
}

fn main() {
    let len1 = std::mem::size_of::<MyString>();
    let len2 = std::mem::size_of::<MiniString>();
    println!("Len: MyString {}, MiniString {}", len1, len2);

    let s1: MyString = "hello world".into();
    let s2: MyString = "这是一个超过了三十个字节的很长很长的字符串".into();
    println!("s1: {:?}, s2: {:?}", s1, s2);
    println!(
        "s1: {}({} bytes, {} chars), s2: {}({} bytes, {} chars)",
        s1,
        s1.len(),
        s1.chars().count(),
        s2,
        s2.len(),
        s2.chars().count()
    );

    let s = String::from("这是一个超过了三十个字节的很长很长的字符串");
    println!("s: {:p}", &*s);
    let s3: MyString = s.into();
    println!("s3: {:p}", &*s3);

    let mut s4: MyString = "Hello Tyr!".into();
    println!("s4: {:?}", s4);
    s4.push_str("这是一个超过了三十个字节的很长很长的字符串");
    println!("s4: {:?}", s4);
}
