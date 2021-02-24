/*
Since a trait object can contain references, the lifetimes of those references
need to be expressed as part of the trait object. This lifetime is written as
Trait + 'a

https://doc.rust-lang.org/reference/lifetime-elision.html#default-trait-object-lifetimes
*/

trait A {
  fn value(&self) -> &str;
}

// 等价于
// impl dyn A + 'static
impl dyn A {
  fn trait_value(&self) -> &str {
    self.value()
  }
}

fn extract1(a: &dyn A) -> &str {
  a.trait_value()
}

fn main() {}
