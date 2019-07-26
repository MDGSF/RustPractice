use std::collections::BinaryHeap;

fn main() {
    let mut heap = BinaryHeap::new();
    assert_eq!(heap.peek(), None);

    println!("len = {}", heap.len());

    heap.push(93);
    heap.push(80);
    heap.push(48);
    heap.push(53);
    heap.push(72);
    heap.push(30);
    heap.push(18);
    heap.push(36);
    heap.push(15);
    heap.push(35);
    heap.push(45);

    println!("len = {}", heap.len());

    assert_eq!(heap.peek(), Some(&93));
    println!("len = {}", heap.len());
    println!("{:?}", heap);

    assert_eq!(heap.pop(), Some(93));
    println!("{:?}", heap);
}
