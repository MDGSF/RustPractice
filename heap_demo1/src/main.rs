use heap_demo1::heap::Heap;

fn main() {
    println!("Hello, world!");
    let mut h = Heap::new();
    h.push(5);
    h.push(4);
    h.push(3);
    h.push(2);
    h.push(1);
}
