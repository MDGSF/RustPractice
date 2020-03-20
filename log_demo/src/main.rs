/*
RUST_LOG=error cargo run
RUST_LOG=info cargo run
RUST_LOG=trace cargo run

RUST_LOG_STYLE=auto
RUST_LOG_STYLE=always
RUST_LOG_STYLE=never

*/

#[macro_use]
extern crate log;

extern crate chrono;

use chrono::prelude::*;
use log::Level;
use std::io::Write;

fn main() {
  //env_logger::init();
  //env_logger::builder().format_timestamp(None).init();

  env_logger::builder()
    .format(|buf, record| {
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
      writeln!(
        buf,
        "{} {}/{}/{} {}:{}:{} {}:{}:{} {}",
        level,
        local.year(),
        local.month(),
        local.day(),
        local.hour(),
        local.minute(),
        local.second(),
        module_path,
        file,
        line,
        record.args()
      )
    })
    .init();

  println!("filename: {}", file!());
  println!("line: {}", line!());
  println!("column: {}", column!());

  dbg!("This is dbg message");

  error!("This is error message");
  warn!("This is warn message");
  info!("This is info message, {}", log_enabled!(Level::Info));
  debug!("This is a debug {}", "message");
  trace!("This is a trace message");

  log!(target: "testlog", Level::Error, "This is error log");
}
