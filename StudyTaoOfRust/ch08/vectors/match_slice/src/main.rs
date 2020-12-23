fn sum(num: &[i32]) {
    match num {
        [one] => println!(" at least two"),
        [first, second] => println!("{:?} + {:?} = {:?}", first, second, first + second),
        _ => println!("sum is {:?}", num.iter().fold(0, |sum, i| sum + i)),
    }
}

fn main() {
    sum(&[1]);
    sum(&[1, 2]);
    sum(&[1, 2, 3]);
    sum(&[1, 2, 3, 5]);
}
