/*
下面的生命周期参数标注表示参数 x,参数 y 和返回值都有一样的生命周期。
返回值的生命周期至少和 min(参数 x，参数 y) 一样长。
*/
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

fn main() {
  let string1 = String::from("abcd");
  let string2 = "xyz";

  let result = longest(string1.as_str(), string2);
  println!("The longest string is {}", result);
}
