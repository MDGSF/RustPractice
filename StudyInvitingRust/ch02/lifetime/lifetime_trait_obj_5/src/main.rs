/*
// For the following trait...
trait Foo { }

// These two are the same as Box<T> has no lifetime bound on T
type T1 = Box<dyn Foo>;
type T2 = Box<dyn Foo + 'static>;

// ...and so are these:
impl dyn Foo {}
impl dyn Foo + 'static {}

// ...so are these, because &'a T requires T: 'a
type T3<'a> = &'a dyn Foo;
type T4<'a> = &'a (dyn Foo + 'a);

// std::cell::Ref<'a, T> also requires T: 'a, so these are the same
type T5<'a> = std::cell::Ref<'a, dyn Foo>;
type T6<'a> = std::cell::Ref<'a, dyn Foo + 'a>;

*/

trait Foo {}

// This is an example of an error.
struct TwoBounds<'a, 'b, T: ?Sized + 'a + 'b> {
  f1: &'a i32,
  f2: &'b i32,
  f3: T,
}
//type T7<'a, 'b> = TwoBounds<'a, 'b, dyn Foo>;
//                                  ^^^^^^^
// Error: the lifetime bound for this object type cannot be deduced from context

type T7<'a, 'b, 'c> = TwoBounds<'a, 'b, dyn Foo + 'c>;

fn main() {}
