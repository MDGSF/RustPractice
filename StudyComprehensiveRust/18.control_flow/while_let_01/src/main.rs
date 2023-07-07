fn main() {
    test01();
    test02();
}

fn test01() {
    let v = vec![10, 20, 30];
    let mut iter = v.into_iter();
    while let Some(x) = iter.next() {
        println!("x: {x}");
    }
}

fn test02() {
    let v = vec![10, 20, 30];
    let mut iter = v.into_iter();
    while true {
        if let Some(x) = iter.next() {
            println!("x: {x}");
        } else {
            break;
        }
    }
}
