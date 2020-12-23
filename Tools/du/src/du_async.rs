use std::env::args;

use async_std::fs;
use async_std::io;
use async_std::path::Path;
use async_std::prelude::*;
use async_std::task;

use async_recursion::async_recursion;
use futures::future::{BoxFuture, FutureExt};

fn main() -> io::Result<()> {
  let root_path = args().nth(1).expect("missing path argument");
  let mut nfiles = 0;
  let mut nbytes = 0;

  task::block_on(du(Path::new(&root_path), &mut nfiles, &mut nbytes))?;

  println!(
    "{} files, {} bytes, {} GB",
    nfiles,
    nbytes,
    nbytes as f64 / 1e9
  );
  Ok(())
}

// #[async_recursion]
fn du(
  path: &Path,
  mut nfiles: &mut u64,
  mut nbytes: &mut u64,
) -> BoxFuture<io::Result<()>> {
  async move {
    let mut entry_iter = fs::read_dir(path).await.expect(&format!("{:?}", path));
    while let Some(entry) = entry_iter.next().await {
      let entry = match entry {
        Ok(entry) => entry,
        Err(e) => {
          eprintln!("invalid entry, err = {}", e);
          continue;
        }
      };

      let entry_path = entry.path();
      if !entry_path.exists().await {
        continue;
      }

      let metadata = match entry_path.metadata().await {
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
        //task::spawn(async {
        du(&entry_path, &mut nfiles, &mut nbytes).await.unwrap();
      //}).await;
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
  .boxed()
}
