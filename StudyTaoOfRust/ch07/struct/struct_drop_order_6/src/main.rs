struct PrintDrop(&'static str);

impl Drop for PrintDrop {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}

enum E {
    Foo(PrintDrop, PrintDrop),
}

struct Foo {
    x: PrintDrop,
    y: PrintDrop,
    z: PrintDrop,
}

fn main() {
    let e = E::Foo(PrintDrop("a"), PrintDrop("b"));
    let foo = Foo {
        x: PrintDrop("x"),
        y: PrintDrop("y"),
        z: PrintDrop("z"),
    };
}
