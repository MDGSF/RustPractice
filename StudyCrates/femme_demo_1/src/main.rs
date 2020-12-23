use kv_log_macro as log;

fn main() {
  femme::with_level(femme::LevelFilter::Trace);
  log::error!("error log");
  log::warn!("warn log");
  log::info!("info log");
  log::debug!("debug log");
  log::trace!("trace log");

  log::info!("info log", {
    name: "huangjian",
    age: 100000,
  });
}
