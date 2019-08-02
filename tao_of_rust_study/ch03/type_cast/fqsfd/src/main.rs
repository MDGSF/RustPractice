struct S(i32);

trait A {
    fn test(&self, i: i32);
}

trait B {
    fn test(&self, i: i32);
}

impl A for S {
    fn test(&self, i: i32) {
        println!("From A: {:?}", i);
    }
}

impl B for S {
    fn test(&self, i: i32) {
        println!("From B: {:?}", i + 1);
    }
}

fn main() {
    let s = S(1);
    A::test(&s, 1);
    B::test(&s, 1);
    <S as A>::test(&s, 1);
    <S as B>::test(&s, 1);

    let a: &'static str = "hello";
    let b: &str = a as &str;
    let c: &'static str = b as &'static str;
    println!("c = {}", c);
}
