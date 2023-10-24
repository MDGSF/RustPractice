use std::env;
use std::process::Stdio;
use tokio::io::AsyncWriteExt;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let working_directory = env::current_dir()?;
    let cmd_name = working_directory.as_path().join("longouterr");

    let mut child = Command::new(cmd_name)
        .arg("hello")
        .arg("world")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn");

    let mut stdin = child
        .stdin
        .take()
        .expect("child did not have a handle to stdin");

    let stdout = child
        .stdout
        .take()
        .expect("child did not have a handle to stdout");

    let stderr = child
        .stderr
        .take()
        .expect("child did not have a handle to stderr");

    tokio::spawn(async move {
        loop {
            stdin
                .write("--- in ---\n".as_bytes())
                .await
                .expect("could not write to stdin");
            sleep(Duration::from_millis(1000)).await;
        }
    });

    let stdout_task = tokio::spawn(async move {
        let mut stdout_reader = BufReader::new(stdout);
        let mut line = String::new();
        loop {
            line.clear();
            match stdout_reader.read_line(&mut line).await {
                Err(err) => return Err(err),
                Ok(0) => return Ok(()),
                Ok(_) => {
                    print!("{}", line);
                }
            }
        }
    });

    let stderr_task = tokio::spawn(async move {
        let mut stderr_reader = BufReader::new(stderr);
        let mut line = String::new();
        loop {
            line.clear();
            match stderr_reader.read_line(&mut line).await {
                Err(err) => return Err(err),
                Ok(0) => return Ok(()),
                Ok(_) => {
                    print!("{}", line);
                }
            }
        }
    });

    let status = child
        .wait()
        .await
        .expect("child process encountered an error");
    println!("child status was: {}", status);

    stdout_task.await.expect("stdout task failed")?;
    stderr_task.await.expect("stderr task failed")?;

    Ok(())
}
