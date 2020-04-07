use std::collections::{BTreeMap, HashMap};
use std::ffi::OsStr;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::ops::Range;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Deserializer;

use crate::{KvsError, Result};

/// The `KvStore` stores string key/value pairs.
///
/// Key/value pairs are stored in a `HashMap` in memory and not persisted to disk.
///
/// Example:
///
/// ```rust
/// # use kvs::KvStore;
/// let mut store = KvStore::new();
/// store.set("key".to_owned(), "value".to_owned());
/// let val = store.get("key".to_owned());
/// assert_eq!(val, Some("value".to_owned()));
/// ```
pub struct KvStore {
  // directory for the log and other data.
  path: PathBuf,

  // map <generation number, file reader>.
  readers: HashMap<u64, BufReaderWithPos<File>>,

  // writer of the current log.
  writer: BufWriteWithPos<File>,

  // current generation number.
  current_gen: u64,

  // map <key, { gen, pos, len }>
  index: BTreeMap<String, CommandPos>,
}

impl KvStore {
  /// Opens a `KvStore` with the given path.
  ///
  /// This will create a new directory if the given one does not exist.
  ///
  /// # Errors
  ///
  /// It propagates I/O or deserialization errors during the log replay.
  pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
    let path = path.into();
    fs::create_dir_all(&path)?;

    let mut readers = HashMap::new();
    let mut index = BTreeMap::new();

    Ok(KvStore { path })
  }

  /// Sets the value of a string key to a string.
  ///
  /// If the key already exists, the previous value will be overwritten.
  pub fn set(&mut self, key: String, value: String) -> Result<()> {
    panic!()
  }

  /// Gets the string value of a given string key.
  ///
  /// Returns `None` if the given key does not exist.
  pub fn get(&mut self, key: String) -> Result<Option<String>> {
    panic!()
  }

  /// Remove a given key.
  pub fn remove(&mut self, key: String) -> Result<()> {
    panic!()
  }

  pub fn compact(&mut self) -> Result<()> {
    panic!()
  }
}

fn sorted_gen_list(path: &Path) -> Result<Vec<u64>> {}

/// Struct representing a command.
#[derive(Serialize, Deserialize, Debug)]
enum Command {
  Set { key: String, value: String },
  Remove { key: String },
}

impl Command {
  fn set(key: String, value: String) -> Command {
    Command::Set { key, value }
  }

  fn remove(key: String) -> Command {
    Command::Remove { key }
  }
}

/// Represents the position and length of a json-serialized command in the log.
struct CommandPos {
  gen: u64,
  pos: u64,
  len: u64,
}

impl From<(u64, Range<u64>)> for CommandPos {
  fn from((gen, range): (u64, Range<u64>)) -> Self {
    CommandPos {
      gen,
      pos: range.start,
      len: range.end - range.start,
    }
  }
}

struct BufReaderWithPos<R: Read + Seek> {
  reader: BufReader<R>,
  pos: u64,
}

impl<R: Read + Seek> BufReaderWithPos<R> {
  fn new(mut inner: R) -> Result<Self> {
    let pos = inner.seek(SeekFrom::Current(0))?;
    Ok(BufReaderWithPos {
      reader: BufReader::new(inner),
      pos,
    })
  }
}

impl<R: Read + Seek> Read for BufReaderWithPos<R> {
  fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
    let len = self.reader.read(buf)?;
    self.pos += len as u64;
    Ok(len)
  }
}

impl<R: Read + Seek> Seek for BufReaderWithPos<R> {
  fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
    self.pos = self.reader.seek(pos)?;
    Ok(self.pos)
  }
}

struct BufWriteWithPos<W: Write + Seek> {
  writer: BufWriter<W>,
  pos: u64,
}

impl<W: Write + Seek> BufWriteWithPos<W> {
  fn new(mut inner: W) -> Result<Self> {
    let pos = inner.seek(SeekFrom::Current(0))?;
    Ok(BufWriteWithPos {
      writer: BufWriter::new(inner),
      pos,
    })
  }
}

impl<W: Write + Seek> Write for BufWriteWithPos<W> {
  fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
    let len = self.writer.write(buf)?;
    self.pos += len as u64;
    Ok(len)
  }

  fn flush(&mut self) -> io::Result<()> {
    self.writer.flush()
  }
}

impl<W: Write + Seek> Seek for BufWriteWithPos<W> {
  fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
    self.pos = self.writer.seek(pos)?;
    Ok(self.pos)
  }
}
