trait FooTrait {
  fn show(&self) {}
}

impl<'a> FooTrait for &'a dyn for<'b> FooTrait
where
  for<'b> dyn FooTrait: FooTrait,
{
  fn show(self: &&'a dyn for<'b> FooTrait) {
    println!("show 1")
  }
}

impl FooTrait for for<'a> fn(&'a dyn for<'b> FooTrait) {
  fn show(&self) {
    println!("show 2")
  }
}

fn global_test(x: &dyn for<'a> FooTrait) {
  (&x).show(); // show 1
  x.show(); // show 2
  <&dyn for<'a> FooTrait as FooTrait>::show(&x); // show 1
}

fn main() {
  let x = &(global_test as for<'a> fn(&'a dyn for<'b> FooTrait));
  global_test(x);
}
