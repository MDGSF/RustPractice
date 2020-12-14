use crate::*;

pub fn init_routers(app: &mut tide::Server<()>) {
  app.at("/").get(|_| async { Ok("Hello, world!") });
  app.at("/get_all_c1_account").get(get_all_c1_account);
}
