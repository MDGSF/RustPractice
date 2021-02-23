struct A<'a> {
  name: &'a str,
}

impl<'a> A<'a> {
  // 等价于 fn get<'b>(&'b self) -> &'b str
  // 为啥这里是 'b 而不是 'a 呢？
  // 'a 是 name 引用的 s 的生命周期
  // 'b 是 A 实例化出来的对象 a 的生命周期
  // 根据生命周期自动推断规则，这个函数的返回值和对象 a 生命周期一样
  fn get(&self) -> &str {
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
