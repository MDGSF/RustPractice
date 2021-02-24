trait A {
  fn value(&self) -> &str;
}

impl dyn A {
  fn trait_value(&self) -> &str {
    self.value()
  }
}

// 可以这么标注生命周期参数
fn extract<'t>(a: &'t (dyn A + 'static)) -> &'t str {
  a.trait_value()
}

fn main() {}
