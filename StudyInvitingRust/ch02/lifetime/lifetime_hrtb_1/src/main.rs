trait For {
  fn four(&self) {}
}

impl<'four> For for &'four for<'fore> For
where
  for<'fore> For: For,
{
  fn four(self: &&'four for<'fore> For) {
    print!("four")
  }
}

impl For for for<'four> fn(&'four for<'fore> For) {
  fn four(&self) {
    print!("for")
  }
}

fn four(four: &for<'four> For) {
  <&for<'four> For as For>::four(&{
    ((&four).four(), four.four());
    four
  })
}

fn main() {
  four(&(four as for<'four> fn(&'four for<'fore> For)))
}
