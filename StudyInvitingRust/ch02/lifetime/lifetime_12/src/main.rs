trait Foo<'a> {}

struct FooImpl<'a> {
  s: &'a [u32],
}

impl<'a> Foo<'a> for FooImpl<'a> {}

fn foo<'a>(s: &'a [u32]) -> Box<dyn Foo<'a> + 'a> {
  Box::new(FooImpl { s: s })
}

fn main() {}
