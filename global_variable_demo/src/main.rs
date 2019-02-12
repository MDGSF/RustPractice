static mut GG: Option<User> = None;

#[derive(Debug)]
struct User {
    id: i32,
}

fn main() {
    {
        let u = User { id: 10 };
        println!("{:?}", u);

        unsafe {
            GG = Some(u);
        }
    }

    unsafe {
        println!("{:?}", GG);

        let r1 = &mut GG as *mut Option<User>;
        println!("r1 = {:p}", r1);
    }
}
