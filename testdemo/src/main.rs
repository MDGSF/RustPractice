trait DemoTrait<T> {
    fn demotrait(&self) -> T;
}

struct Student {
    name: String,
}

impl<T> DemoTrait<T> for Student
where
    T: Default,
{
    fn demotrait(&self) -> T {
        let a: T = Default::default();
        a
    }
}

fn main() {
    let s = Student {
        name: "".to_string(),
    };
    let a: i32 = s.demotrait();
    println!("a = {}", a);
}
