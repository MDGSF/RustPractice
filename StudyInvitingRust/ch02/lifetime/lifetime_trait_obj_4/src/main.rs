trait A {
  fn value(&self) -> &str;
}

impl<'a> dyn A + 'a {
  fn trait_value(&self) -> &str {
    self.value()
  }
}

fn extract4<'t, 'm>(a: &'t (dyn A + 'm)) -> &'t str {
  a.trait_value()
}

fn main() {}
