fn main() {
    println!("Hello, world!");

    println!("10 is: {}", is_palindrome(10));
    println!("121 is: {}", is_palindrome(121));
    println!("1 is: {}", is_palindrome(1));
    println!("1111 is: {}", is_palindrome(1111));
}

pub fn is_palindrome(x: i32) -> bool {
    let mut newx = x;
    let mut r = 0;

    while newx > 0 {
        let t = newx % 10;
        newx = newx / 10;
        r = r * 10 + t;
    }

    if r == x {
        true
    } else {
        false
    }
}
