#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
}

impl Test {
    fn new(txt: &str) -> Box<Self> {
        Box::new(Test {
            a: String::from(txt),
            b: std::ptr::null(),
        })
    }

    fn init(self: &mut Self) {
        let self_ptr: *const String = &self.a;
        self.b = self_ptr;
    }

    fn a(self: &Self) -> &str {
        &self.a
    }

    fn b(self: &Self) -> &String {
        unsafe { &*self.b }
    }
}

fn main() {
    println!("Hello, world!");

    let mut test1 = Test::new("test1");
    Test::init(test1.as_mut());

    let mut test2 = Test::new("test2");
    Test::init(test2.as_mut());

    println!("test1: {}, {}", test1.a(), test1.b());
    println!("test2: {}, {}", test2.a(), test2.b());

    std::mem::swap(&mut test1, &mut test2);
    println!("test1: {}, {}", test1.a(), test1.b());
    println!("test2: {}, {}", test2.a(), test2.b());
}
