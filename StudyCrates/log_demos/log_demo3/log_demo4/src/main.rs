use kv_log_macro::*;

fn main() {
  femme::start();

  info!("hello");
  info!("hello",);
  info!("hello {}", "cats");
  info!("hello {}", "cats",);
  info!("hello {}", "cats", {
      cat_1: "chashu",
      cat_2: "nori",
  });
}
