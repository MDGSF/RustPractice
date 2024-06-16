fn sum_of_val(nums: &[i32], num: i32) -> i32 {
    let mut sum: i32 = 0;
    for n in nums {
        sum += n;
    }
    sum + num
}

fn main() {
    let num = 10;
    let nums = [1, 2, 3, 4, 5, 6, 7, 8];
    let sum = sum_of_val(&nums, num);
    println!("sum is {sum}");
}
