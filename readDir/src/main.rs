use std::{ffi::OsStr, fs, io};

fn main() -> io::Result<()> {
  let mut result: Vec<_> = fs::read_dir(".")?
    .flatten()
    .map(|entry| entry.path())
    .filter(|pathbuf| pathbuf.is_file() && pathbuf.extension() == Some("log".as_ref()))
    .map(|pathbuf| {
      pathbuf
        .file_name()
        .and_then(OsStr::to_str)
        .map(|s| s.trim_end_matches(".log"))
        .map(str::parse::<u64>)
    })
    .flatten()
    .flatten()
    .collect();
  //let a: u8 = result;
  result.sort();
  println!("{:?}", result);
  Ok(())
}
