struct Foo;

impl Foo {
    fn mutate_and_share(&mut self) -> &Self {
        &*self
    }

    fn share(&self) {}
}

fn main() {
    let mut foo = Foo;
    let loan = foo.mutate_and_share(); // 这里使用了 &mut self
    foo.share(); // 这里使用了 &self
}
