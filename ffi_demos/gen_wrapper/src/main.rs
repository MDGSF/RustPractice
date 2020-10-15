use anyhow::Result;
use std::env;
use walkdir::WalkDir;

fn main() -> Result<()> {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    println!("usage: gen_wrapper <path>");
    std::process::exit(0);
  }
  let path = &args[1];
  for entry in WalkDir::new(&path)
    .follow_links(true)
    .into_iter()
    .filter_map(|e| e.ok())
  {
    let f_name = entry.file_name().to_string_lossy();
    if f_name.ends_with(".h") {
      // println!("{:?}, {}", entry.path(), f_name);
      println!("#include {:?}", entry.path());
    }
  }

  Ok(())
}
