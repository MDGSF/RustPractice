fn test1(mut a: [i32; 3]) {
    a[0] = 10;
    println!("test1 a = {:?}", a); // test1 a = [10, 2, 3]
}

fn test2(mut b: Vec<i32>) {
    b[0] = 20;
    println!("test2 b = {:?}", b); // test2 b = [20, 2, 3]
}

fn test3(c: &mut [i32]) {
    c[0] = 30;
    println!("test3 c = {:?}", c); // test3 c = [30, 2, 3]
}

fn main() {
    // a 默认实现 Copy trait，所以在传入 test1 函数时执行了按位拷贝整个数组。
    let a = [1, 2, 3];
    test1(a);
    println!("main a = {:?}", a); // main a = [1, 2, 3]

    // b 的所有权转移到了函数 test2 中去了。
    let b = vec![1, 2, 3];
    test2(b);
    // println!("main b = {:?}", b);

    // 推荐这么使用，在拥有所有权的地方使用 vector 动态数组，
    // 在租借出去的时候，使用 mutable slice 来接收。
    let mut c = vec![1, 2, 3];
    test3(&mut c);
    println!("main c = {:?}", c); // main c = [30, 2, 3]
}
