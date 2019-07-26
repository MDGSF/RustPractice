struct Student {
    name: String,
}

impl Student {
    fn xx(this: Self) {
        println!("{}", this.name);
    }
}

fn main() {
    let s = Student {
        name: "hj".to_string(),
    };
    Student::xx(s);
}
