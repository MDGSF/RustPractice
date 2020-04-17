use std::env::args;
use std::fs;
use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
  let root_path = args().nth(1).expect("missing path argument");
  let mut nfiles = 0;
  let mut nbytes = 0;
  du(Path::new(&root_path), &mut nfiles, &mut nbytes)?;
  println!(
    "{} files, {} bytes, {} GB",
    nfiles,
    nbytes,
    nbytes as f64 / 1e9
  );
  Ok(())
}

fn du(path: &Path, mut nfiles: &mut u64, mut nbytes: &mut u64) -> io::Result<()> {
  let entry_iter = fs::read_dir(path).expect(&format!("{:?}", path));
  for entry in entry_iter {
    let entry = match entry {
      Ok(entry) => entry,
      Err(e) => {
        eprintln!("invalid entry, err = {}", e);
        continue;
      }
    };

    let entry_path = entry.path();
    if !entry_path.exists() {
      continue;
    }

    let metadata = match entry_path.metadata() {
      Ok(metadata) => metadata,
      Err(e) => {
        eprintln!("invalid metadata, err = {}", e);
        continue;
      }
    };

    let file_size = metadata.len();
    let file_type = metadata.file_type();

    if file_type.is_symlink() {
      continue;
    } else if file_type.is_dir() {
      du(&entry_path, &mut nfiles, &mut nbytes)?;
    } else {
      *nfiles += 1;
      *nbytes += file_size;

      if *nfiles > 0 && *nfiles % 100 == 0 {
        println!(
          "{} files, {} bytes, {} GB",
          nfiles,
          nbytes,
          *nbytes as f64 / 1e9
        );
      }
    }
  }
  Ok(())
}
