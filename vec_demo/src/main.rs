fn main() {
    test1()
}

fn test1() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);

    println!("vec.len() = {}", vec.len());
    println!("vec[0] = {}", vec[0]);
    println!("vec[1] = {}", vec[1]);

    let top_elem = vec.pop();
    if top_elem == Some(2) {
        println!("top = {:?}", top_elem);
    }
    println!("vec.len() = {}", vec.len());

    vec[0] = 7;
    println!("vec = {:?}", vec);

    vec.extend([1, 2, 3].iter().cloned());
    println!("vec = {:?}", vec);

    for x in &vec {
        println!("{}", x);
    }
    assert_eq!(vec, [7, 1, 2, 3]);
}

fn test2() {}
