use std::fmt::Display;

struct User {
    name: String,
}

impl User {
    fn announce_and_return<'a>(&self, announcement: &'a str) -> &'a str {
        announcement
    }
}

// 引用的生命周期标注 'a  和泛型 T 都是放在 <'a, T>  这个尖括号中的。
// @param x: &str
// @param y: &str
// @param ann: T 为实现了 Display 这个 trait 的泛型
// @return &str
//
// 参数 x, 参数 y, 返回值 都具有相同的生命周期
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    println!("Hello, world!");
}
