#[macro_use]
extern crate log;
extern crate chrono;

use chrono::prelude::*;
use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};

pub struct SimpleLogger {
  level: Level,
}

impl log::Log for SimpleLogger {
  fn enabled(&self, metadata: &Metadata) -> bool {
    metadata.level() <= self.level
  }
  fn log(&self, record: &Record) {
    if !self.enabled(record.metadata()) {
      return;
    }
    let level = match record.level() {
      Level::Error => "ERROR",
      Level::Warn => " WARN",
      Level::Info => " INFO",
      Level::Debug => "DEBUG",
      Level::Trace => "TRACE",
    };
    let module_path = match record.module_path() {
      Some(module_path) => module_path,
      None => "",
    };
    let file = match record.file() {
      Some(file) => file,
      None => "",
    };
    let line = match record.line() {
      Some(line) => line,
      None => 0,
    };
    let local: DateTime<Local> = Local::now();
    println!(
      "{} {}/{}/{} {}:{}:{} {}:{}:{}:{} {}",
      level,
      local.year(),
      local.month(),
      local.day(),
      local.hour(),
      local.minute(),
      local.second(),
      record.target(),
      module_path,
      file,
      line,
      record.args()
    )
  }
  fn flush(&self) {}
}

pub fn init() -> Result<(), SetLoggerError> {
  let logger = SimpleLogger { level: Level::Error };

  let r = log::set_boxed_logger(Box::new(logger));

  if r.is_ok() {
    log::set_max_level(LevelFilter::Trace);
  }

  r
}

fn main() {
  init().expect("init failed");
  error!("Hello, world!");
  warn!("Hello, world!");
  info!("Hello, world!");
  debug!("Hello, world!");
  trace!("Hello, world!");
}
