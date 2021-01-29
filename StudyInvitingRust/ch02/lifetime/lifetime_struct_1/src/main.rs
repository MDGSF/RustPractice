// 'a 意味着 ImportantExcerpt 实例化出来的对象的生命周期
// 不能超过成员 part 所引用的对象的生命周期
// 例如：
// 下面 main 函数中的 i 的生命周期不能超过 noval 的生命周期。
#[derive(Debug)]
struct ImportantExcerpt<'a> {
  part: &'a str,
}

fn main() {
  let noval = String::from("Call me Ishmael. Some years ago...");
  let first_sentence = noval.split('.').next().expect("Could not find a '.'");
  let i = ImportantExcerpt {
    part: first_sentence,
  };
  println!("i = {:?}", i);
}
