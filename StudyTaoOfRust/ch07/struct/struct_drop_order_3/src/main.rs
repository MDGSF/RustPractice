struct PrintDrop(&'static str);

impl Drop for PrintDrop {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}

fn main() {
    let z = PrintDrop("z");
    let x = PrintDrop("x");
    let y = PrintDrop("y");
    let closure = move || {
        y;
        z;
        x;
    };
}
