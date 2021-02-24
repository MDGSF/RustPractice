trait A {
  fn value(&self) -> &str;
}

impl dyn A {
  fn trait_value(&self) -> &str {
    self.value()
  }
}

type DynA = dyn A;
fn extract3(a: &DynA) -> &str {
  a.trait_value()
}

fn main() {}
