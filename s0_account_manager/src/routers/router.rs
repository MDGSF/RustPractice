use crate::*;

pub fn init_routers(app: &mut tide::Server<State>) {
  app.at("/").get(|_| async { Ok("Hello, world!") });
  app.at("/add_c1_account").post(add_c1_account);
  app.at("/del_c1_account").post(del_c1_account);
  app.at("/get_all_c1_account").get(get_all_c1_account);
}
