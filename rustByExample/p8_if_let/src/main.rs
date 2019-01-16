fn main() {
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    if let Some(i) = letter {
        println!("Matched: {:?}!", i);
    } else {
        println!("Didn't match a number, Let's go with a letter!");
    };

    let i_like_letters = false;

    if let Some(i) = emoticon {
    } else if i_like_letters {
    } else {
    };

    test1();
}

enum Foo {
    Bar,
    Baz,
    Qux(u32),
}

fn test1() {
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    if let Foo::Bar = a {
        println!("a is a foobar");
    }

    if let Foo::Bar = b {
        println!("b is foobar");
    }

    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }
}
