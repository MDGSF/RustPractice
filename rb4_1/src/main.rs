fn main() {
    println!("Hello, world!");

    let x: i32 = 17;
    {
        let y: i32 = 3;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    //println!("The value of x is {} and value of y is {}", x, y);
}
