use std::io::prelude::*;
use std::io::{BufReader, Error, ErrorKind, Write};
use std::process::{Command, Stdio};
use std::time;
use std::{env, io};

fn main() -> Result<(), Error> {
    let working_directory = env::current_dir()?;
    let cmd_name = working_directory.as_path().join("longouterr");

    let mut child = Command::new(cmd_name)
        .current_dir(env::current_dir()?)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    std::thread::spawn(move || loop {
        stdin
            .write_all("---- in ----\n".as_bytes())
            .expect("Failed to write to stdin");
        std::thread::sleep(time::Duration::from_millis(1000));
    });

    let stdout = child
        .stdout
        .as_mut()
        .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard output."))?;
    let mut stdout_reader = BufReader::new(stdout);

    let stderr = child
        .stderr
        .as_mut()
        .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard error output."))?;
    let mut stderr_reader = BufReader::new(stderr);

    loop {
        let (stdout_bytes, stderr_bytes) =
            match (stdout_reader.fill_buf(), stderr_reader.fill_buf()) {
                (Ok(stdout), Ok(stderr)) => {
                    io::stdout().write(stdout).unwrap();
                    io::stderr().write(stderr).unwrap();

                    (stdout.len(), stderr.len())
                }
                other => panic!("Some better error handling here... {:?}", other),
            };

        if stdout_bytes == 0 && stderr_bytes == 0 {
            // Seems less-than-ideal; should be some way of
            // telling if the child has actually exited vs just
            // not outputting anything.
            break;
        }

        stdout_reader.consume(stdout_bytes);
        stderr_reader.consume(stderr_bytes);
    }

    let status = child.wait().expect("Waiting for child failed");
    println!("Finished with status {:?}", status);

    Ok(())
}
