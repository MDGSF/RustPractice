use std::process::Command;

fn main() {
  let output = Command::new("echo")
    .arg("Hello World")
    .output()
    .expect("Failed to execute command");
  println!("{:?}", output.stdout.as_slice());
}
