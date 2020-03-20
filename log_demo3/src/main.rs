#[macro_use]
extern crate log;
extern crate simplelog;

use simplelog::*;

fn main() {
  let config = ConfigBuilder::new()
    .set_thread_level(LevelFilter::Off)
    .set_target_level(LevelFilter::Off)
    .set_location_level(LevelFilter::Error)
    .set_time_format_str("%Y/%m/%d %H:%M:%S")
    .set_time_to_local(true)
    .build();
  SimpleLogger::init(LevelFilter::Trace, config).unwrap();
  //SimpleLogger::init(LevelFilter::Trace, Config::default()).unwrap();

  error!("Bright red error");
  warn!("This is warn message");
  info!("This only appears in the log file");
  debug!("This level is currently not enabled for any logger");
  trace!("This is trace message");
}
