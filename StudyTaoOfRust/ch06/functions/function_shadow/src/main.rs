fn f() {
    println!("1");
}

fn r#match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}

fn main() {
    f();

    {
        f();

        fn f() {
            println!("3");
        }
    }

    f();

    fn f() {
        println!("2");
    }

    assert!(r#match("foo", "foobar"));
}
