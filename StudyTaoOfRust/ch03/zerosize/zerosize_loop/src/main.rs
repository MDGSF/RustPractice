fn main() {
    // 查看变量类型
    // let v: () = vec![(); 10];

    let v: Vec<()> = vec![(); 10];
    for i in v {
        println!("{:?}", i);
    }
}
