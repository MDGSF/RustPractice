// 那如果返回值只和参数 x 有关，那么只需要给返回值和参数 x 手动标注生命周期。
//
// 下面的函数等价于：
// fn longest<'a>(x: &'a str, y: &'b str) -> &'a str
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
  x
}

// 上面的函数也可以写为：
fn longest2<'a: 'c, 'b, 'c>(x: &'a str, y: &'b str) -> &'c str {
  x
}

fn main() {
  let string1 = String::from("abcd");
  let string2 = "xyz";

  let result = longest(string1.as_str(), string2);
  println!("The longest string is {}", result);
}
