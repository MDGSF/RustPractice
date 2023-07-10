use std::fs::File;
use std::io::{self, Read};
use thiserror::Error;

/*
thiserror’s derive macro automatically implements std::error::Error, and optionally Display (if the #[error(...)] attributes are provided) and From (if the #[from] attribute is added). It also works for structs.
*/
#[derive(Debug, Error)]
enum ReadUsernameError {
    #[error("Could not read: {0}")]
    IoError(#[from] io::Error),

    #[error("Found no username in {0}")]
    EmptyUsername(String),
}

fn read_username(path: &str) -> Result<String, ReadUsernameError> {
    let mut username = String::with_capacity(100);
    File::open(path)?.read_to_string(&mut username)?;
    if username.is_empty() {
        return Err(ReadUsernameError::EmptyUsername(String::from(path)));
    }
    Ok(username)
}

fn main() {
    let username = read_username("config.dat");
    println!("{username:?}");
}
