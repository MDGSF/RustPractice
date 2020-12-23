struct PrintDrop(&'static str);

impl Drop for PrintDrop {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}

fn main() {
    let tup1 = (PrintDrop("a"), PrintDrop("b"), PrintDrop("c"));
    let tup2 = (PrintDrop("x"), PrintDrop("y"), panic!());
}
