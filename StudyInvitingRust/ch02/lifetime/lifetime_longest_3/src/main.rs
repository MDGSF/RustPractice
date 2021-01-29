// 下面的生命周期标注添加了：
// 参数 x 与返回值的关系
// 参数 y 与返回值的关系
// 'a >= 'c
// 'b >= 'c
fn longest<'a: 'c, 'b: 'c, 'c>(x: &'a str, y: &'b str) -> &'c str {
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
