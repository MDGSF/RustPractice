fn main() {
    test1();
    test2();
    test3();
}

fn test1() {
    println!("\ntest1 start");

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
        print!("{} ", x);
    }
    println!();

    assert_eq!(vec, [7, 1, 2, 3]);
}

fn test2() {
    println!("\ntest2 start, bubble sort");
    let mut v = vec![3, 2, 5, 1, 4];
    println!("v = {:?}", v);

    let mut i = 0;
    while i < v.len() {
        let mut j = v.len() - 1;
        while j > i {
            if v[j - 1] > v[j] {
                let t = v[j - 1];
                v[j - 1] = v[j];
                v[j] = t;
            }
            j -= 1;
        }
        i += 1;
    }

    println!("after sort, v = {:?}", v);
}

fn test3() {
    println!("\ntest3 start, bubble sort");
    let mut v = vec![3, 2, 5, 1, 4];
    println!("v = {:?}", v);

    bubble_sort(&mut v);
    println!("after sort, v = {:?}", v);
}

fn bubble_sort<T>(v: &mut [T])
where
    T: PartialOrd + Copy,
{
    let mut i = 0;
    while i < v.len() {
        let mut j = v.len() - 1;
        while j > i {
            if v[j - 1] > v[j] {
                v.swap(j - 1, j);
            }
            j -= 1;
        }
        i += 1;
    }
}
