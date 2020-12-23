use crate::*;
use anyhow::Result;

pub fn init_routers(app: &mut tide::Server<State>) -> Result<()> {
  app.at("/").get(get_index);
  app.at("/static").serve_dir("views/static/")?;

  app.at("/add_c1_account").post(add_c1_account);
  app.at("/del_c1_account").post(del_c1_account);
  app.at("/get_all_c1_account").get(get_all_c1_account);
  Ok(())
}
