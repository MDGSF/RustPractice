fn foo<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() % 2 == 0 {
        x
    } else {
        y
    }
}

fn main() {
    let x = String::from("hello");
    let z;
    let y = String::from("world");
    z = foo(&x, &y);
    println!("{:?}", z);
}

/*
 * 在 main 函数中，变量创建顺序为 x, z, y，变量析构顺序为 y, z , x
 * 在 rust 2015 版本中，就认为 z 生存周期比 y 更长，
 * 但是实际上这 3 个变量是在同一个生命周期中的。
 */
