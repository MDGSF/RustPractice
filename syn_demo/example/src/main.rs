use heapsize::HeapSize;

#[derive(HeapSize)]
struct Demo<'a, T: ?Sized> {
    a: Box<T>,
    b: u8,
    c: &'a str,
    d: String,
    sub: Sub,
}

#[derive(HeapSize)]
struct Sub {
    e: String,
}

fn main() {
    let demo = Demo {
        a: b"bytestring".to_vec().into_boxed_slice(),
        b: 255,
        c: "&'static str",
        d: "String".to_string(),
        sub: Sub {
            e: "hello".to_string(),
        },
    };

    println!("heap size = {}", demo.heap_size_of_children());
}
