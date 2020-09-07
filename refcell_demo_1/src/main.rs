use std::cell::RefCell;

#[derive(Debug)]
struct User {
  id: u32,
  year_registered: u32,
  username: String,
  active: RefCell<bool>,
}

fn main() {
  let user_1 = User {
    id: 1,
    year_registered: 2020,
    username: "User 1".to_string(),
    active: RefCell::new(true),
  };

  user_1.active.replace(false);

  println!("{:#?}", user_1);

  let _borrow_one = user_1.active.borrow_mut();
}
