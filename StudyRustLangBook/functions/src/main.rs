fn main() {
    println!("Hello, world!");

    let x = five();
    println!("The value of x is: {}", x);

    let plus_one_value = plus_one(5);
    println!("The value of plus_one is: {}", plus_one_value);

    another_function(5, 6);
}

fn another_function(x: i32, y: i32) {
    println!("Another function, the value of x is: {}.", x);
    println!("Another function, the value of y is: {}.", y);

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
