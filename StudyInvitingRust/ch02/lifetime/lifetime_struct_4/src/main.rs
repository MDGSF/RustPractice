struct A<'a> {
  name: &'a str,
}

impl<'a> A<'a> {
  // 等价于 fn get<'b>(&'b self) -> &'a str
  // 返回值的生命周期和 name 引用的 s 变量一样长
  // 而 'b 则是代表 A 实例化出来的对象的生命周期
  fn get(&self) -> &'a str {
    self.name
  }
}

fn main() {
  let s = String::from("hello");
  let s_ref;

  {
    let a = A { name: &s };
    s_ref = a.get();
  }

  println!("{:?}", s_ref);
}
