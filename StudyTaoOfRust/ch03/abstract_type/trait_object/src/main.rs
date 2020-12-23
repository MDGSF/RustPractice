#[derive(Debug)]
struct Foo;

trait Bar {
    fn baz(&self);
}

impl Bar for Foo {
    fn baz(&self) {
        println!("{:?}", self);
    }
}

fn static_dispatch<T>(t: &T)
where
    T: Bar,
{
    t.baz();
}

fn dynamic_dispatch(t: &dyn Bar) {
    t.baz();
}

fn main() {
    let foo = Foo;
    static_dispatch(&foo);
    dynamic_dispatch(&foo);
}
