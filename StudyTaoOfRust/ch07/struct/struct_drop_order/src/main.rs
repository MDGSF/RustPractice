struct PrintDrop(&'static str);

impl Drop for PrintDrop {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}

fn main() {
    let x = PrintDrop("x");
    let y = PrintDrop("y");
}
