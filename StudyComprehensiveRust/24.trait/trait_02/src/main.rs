trait Pet {
    fn name(&self) -> String;
}

struct Dog {
    name: String,
}

struct Cat;

impl Pet for Dog {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Pet for Cat {
    fn name(&self) -> String {
        String::from("The Cat")
    }
}

fn main() {
    let pets: Vec<Box<dyn Pet>> = vec![
        Box::new(Cat),
        Box::new(Dog {
            name: String::from("Fido"),
        }),
    ];
    for pet in pets {
        println!("Hello {}!", pet.name());
    }

    println!("Dog: {}", std::mem::size_of::<Dog>());
    println!("Cat: {}", std::mem::size_of::<Cat>());
    println!("&Dog: {}", std::mem::size_of::<&Dog>());
    println!("&Cat: {}", std::mem::size_of::<&Cat>());
    println!("&dyn Pet: {}", std::mem::size_of::<&dyn Pet>());
    println!("Box<dyn Pet>: {}", std::mem::size_of::<Box<dyn Pet>>());
}
