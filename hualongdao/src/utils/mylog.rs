use chrono::prelude::*;
use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};
use once_cell::sync::Lazy;
use std::sync::Mutex;

static HAS_INIT_LOG: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

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
      Level::Error => "ERRO",
      Level::Warn => "WARN",
      Level::Info => "INFO",
      Level::Debug => "DEBU",
      Level::Trace => "TRAC",
    };
    let _module_path = match record.module_path() {
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
      "[<{:?}>{}/{:>0width$}/{:>0width$} {:>0width$}:{:>0width$}:{:>0width$}]({}){}({}): {}",
      std::thread::current().id(),
      local.year(),
      local.month(),
      local.day(),
      local.hour(),
      local.minute(),
      local.second(),
      level,
      //record.target(),
      //module_path,
      file,
      line,
      record.args(),
      width = 2,
    )
  }
  fn flush(&self) {}
}

pub fn init_log(loglevel: u32) -> Result<(), SetLoggerError> {
  if *HAS_INIT_LOG.lock().unwrap() {
    return Ok(());
  }
  *HAS_INIT_LOG.lock().unwrap() = true;

  let level = match loglevel {
    1 => Level::Error,
    2 => Level::Warn,
    3 => Level::Info,
    4 => Level::Debug,
    5 => Level::Trace,
    _ => Level::Error,
  };

  let logger = SimpleLogger { level };

  let r = log::set_boxed_logger(Box::new(logger));

  if r.is_ok() {
    log::set_max_level(LevelFilter::Trace);
  }

  r
}
