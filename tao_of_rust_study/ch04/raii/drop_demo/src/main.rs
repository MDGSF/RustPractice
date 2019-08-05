use std::ops::Drop;

#[derive(Debug)]
struct S(i32);

impl Drop for S {
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}

fn main() {
    let x = S(1);
    println!("create x: {:?}", x);
    {
        let y = S(2);
        println!("create y: {:?}", y);
        println!("exit inner scope");
    }

    let x = S(3);

    println!("exit main");
}
