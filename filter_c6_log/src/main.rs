use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufRead;
use std::{fs, io};

fn main() -> Result<()> {
  println!("Hello, world!");

  let log_dir = "/home/huangjian/c6-log/dtc_50ac4ddcce0f8d57".to_string();
  let log_file_prefix =
    "/home/huangjian/c6-log/dtc_50ac4ddcce0f8d57/c6-branch4.log.2020-10-20".to_string();
  let output_file = "/home/huangjian/c6-log/dtc_50ac4ddcce0f8d57/50ac4ddcce0f8d57_2020_10_20.log";

  let mut entries = fs::read_dir(log_dir)?
    .map(|res| res.map(|e| e.path()))
    .map(|res| res.map(|e| e.to_str().unwrap().to_string()))
    .collect::<Result<Vec<_>, io::Error>>()?;

  // The order in which `read_dir` returns entries is not guaranteed. If reproducible
  // ordering is required the entries should be explicitly sorted.

  entries.sort();

  // The entries have now been sorted by their path.

  let mut result = String::new();

  //println!("{:?}", entries);
  for entry in entries.iter() {
    if entry.starts_with(&log_file_prefix) {
      let file = File::open(entry)?;
      let lines = io::BufReader::new(file).lines();
      for line in lines {
        let line = line.unwrap();
        if line.contains("Current device IsM4") {
          let line = line.replace("Current device IsM4", "c6 start");
          result.push_str("\n");
          result.push_str(&line);
          result.push_str("\n");
        }
        if (line.starts_with("ERRO")
          && !line.contains("start.go")
          && !line.contains("sqlite cache (status_queue) overflow max size"))
          || (line.contains("connected to receiver"))
          || (line.contains("connected to minieye_receiver"))
        {
          result.push_str(&line);
          result.push_str("\n");
        }
      }
    }
  }

  // println!("result = {}", result);

  let mut outputf = File::create(output_file)?;
  outputf.write_all(result.as_bytes())?;

  Ok(())
}
