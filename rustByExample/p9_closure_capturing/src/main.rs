fn main() {
    use std::mem;

    let color = "green";

    let print = || println!("`color`: {}", color);

    print();
    print();

    let mut count = 0;
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    inc();
    inc();

    let movable = Box::new(3);

    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    consume();
    //consume();

    test1();
}

fn test1() {
    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    //println!("There're {} elements in vec", haystack.len());
}
