struct PrintDrop(&'static str);

impl Drop for PrintDrop {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}

struct Foo {
    bar: PrintDrop,
    baz: PrintDrop,
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Dropping Foo");
    }
}

fn main() {
    let foo = Foo {
        bar: PrintDrop("bar"),
        baz: PrintDrop("baz"),
    };
}
