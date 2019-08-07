/*
 * Copy on write
 *
 * 被 Cow 包裹的数据有两种情况：
 * 1. 是租借来的数据
 * 2. 拥有数据的所有权
 *
 * 拥有所有权的数据，可以任意修改。而租借来的数据，如果要修改的话，会先执行克隆操作，再修改。
 */

use std::borrow::Cow;

fn abs_all(input: &mut Cow<[i32]>) {
    for i in 0..input.len() {
        let v = input[i];
        if v < 0 {
            input.to_mut()[i] = -v;
        }
    }
}

fn abs_sum(ns: &[i32]) -> i32 {
    let mut lst = Cow::from(ns);
    abs_all(&mut lst);
    lst.iter().fold(0, |acc, &n| acc + n)
}

fn main() {
    // s1 没有被修改，和 i1 公用一份内存
    let s1 = [1, 2, 3];
    let mut i1 = Cow::from(&s1[..]);
    abs_all(&mut i1);
    println!("IN: {:?}", s1);
    println!("OUT: {:?}", i1);

    // 修改了数据，所以 s2 和 i2 用不同的内存。
    let s2 = [1, 2, 3, -45, 5];
    let mut i2 = Cow::from(&s2[..]);
    abs_all(&mut i2);
    println!("IN: {:?}", s2);
    println!("OUT: {:?}", i2);

    // 这里不会发生克隆，因为数据本身就拥有所有权
    let mut v1 = Cow::from(vec![1, 2, -3, 4]);
    abs_all(&mut v1);
    println!("IN/OUT: {:?}", v1);

    // 没有可变需求，所有数据没有发生克隆
    let s3 = [1, 3, 5, 6];
    let sum1 = abs_sum(&s3[..]);
    println!("s3 = {:?}", s3);
    println!("s3 sum = {:?}", sum1);

    let s4 = [1, -3, 5, -6];
    let sum2 = abs_sum(&s4[..]);
    println!("s4 = {:?}", s4);
    println!("s4 sum = {:?}", sum2);
}
