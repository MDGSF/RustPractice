fn foo<'a, 'b>(x: &'a i32, mut y: &'b i32)
where
  'a: 'b,
{
  // &'a i32 is a subtype of &'b i32 because 'a: 'b
  y = x;

  let r: &'b &'a i32 = &&0;

  // 因为 'a > 'b，所以 'a 的生命周期大于 'b 的生命周期
  // 生命周期更长的 'a 不能引用生命周期更短的 'b
  // let r: &'a &'b i32 = &&0;
}

fn main() {
  println!("Hello, world!");
}
