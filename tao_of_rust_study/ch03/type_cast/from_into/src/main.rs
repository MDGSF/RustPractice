#[derive(Debug)]
struct Person {
    name: String,
}

impl Person {
    fn new<T: Into<String>>(name: T) -> Person {
        Person { name: name.into() }
    }
}

fn main() {
    let string = "hello".to_string();
    let other_string = String::from("hello");
    assert_eq!(string, other_string);

    let person1 = Person::new("Alex");
    let person2 = Person::new("Alex".to_string());
    println!("{:?}", person1);
    println!("{:?}", person2);
}
