fn pick(arr: [i32; 3]) {
    match arr {
        [_, _, 3] => println!("ends with 3"),
        [a, 2, c] => println!("{:?}, 2, {:?}", a, c),
        [_, _, _] => println!("pass!"),
    }
}

fn main() {
    let arr = [1, 2, 3];
    pick(arr);
    let arr = [1, 2, 5];
    pick(arr);
    let arr = [1, 3, 5];
    pick(arr);
}
