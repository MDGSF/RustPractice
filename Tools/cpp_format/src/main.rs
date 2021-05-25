extern crate clap;

use anyhow::Result;
use clap::{App, Arg};
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

fn main() -> Result<()> {
  let matches = App::new("cpp_format")
    .version("1.0")
    .author("HuangJian <1342042894@qq.com>")
    .about("Format C++ code")
    .arg(
      Arg::with_name("source_dir")
        .short("i")
        .long("source_dir")
        .value_name("SOURCE_DIR")
        .help("C++ Source Code Directory"),
    )
    .get_matches();

  let source_dir = match matches.value_of("source_dir") {
    Some(source_dir) => PathBuf::from(source_dir),
    None => env::current_dir()?,
  };

  println!("start format directory: {:?}", source_dir);

  walk_dir(&source_dir)?;

  Ok(())
}

fn walk_dir(directory: &Path) -> Result<()> {
  let mut paths: Vec<_> = fs::read_dir(directory)?.map(|r| r.unwrap()).collect();
  paths.sort_by_key(|dir| dir.path());

  for entry in paths {
    let path = entry.path();
    if path.is_dir() {
      walk_dir(&path)?;
    } else if path.is_file() && is_cpp_file(&path)? {
      println!("start format file: {:?}", path);
      astyle_cpp_file(&path)?;
      format_cpp_file(&path)?;
    }
  }

  Ok(())
}

fn is_cpp_file(file_name: &Path) -> Result<bool> {
  let str_file_name = file_name.to_str().unwrap();
  if str_file_name.ends_with(".pb.cc") || str_file_name.ends_with(".pb.h") {
    return Ok(false);
  }
  if let Some(ext) = file_name.extension() {
    let ext = ext.to_str().unwrap().to_lowercase();
    if ext == "cpp"
      || ext == "h"
      || ext == "hpp"
      || ext == "cxx"
      || ext == "cc"
      || ext == "inc"
      || ext == "c"
    {
      return Ok(true);
    }
  }
  Ok(false)
}

fn format_cpp_file(file_name: &Path) -> Result<()> {
  Command::new("clang-format")
    .arg("-i")
    .arg("--fallback-style=Google")
    .arg(file_name)
    .spawn()
    .expect("clang-format C++ Code");
  Ok(())
}

fn astyle_cpp_file(file_name: &Path) -> Result<()> {
  Command::new("astyle")
    .arg("--style=google")
    .arg("--indent=spaces=2")
    .arg("-xb") // 给单行 if 换行
    .arg("-j") // 给单行 if 添加括号
    .arg("-xC80") // 单行最大长度80
    .arg("--lineend=linux") // 每行以 linux (LF) 结尾
    .arg("--suffix=none") // 格式化之后不保留副本
    .arg(file_name)
    .spawn()
    .expect("astyle format C++ Code");
  Ok(())
}
