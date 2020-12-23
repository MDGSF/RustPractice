use std::process::{Command, Stdio};

fn main() {
  let echo_child = Command::new("echo")
    .arg("Oh no, a tpyo!")
    .stdout(Stdio::piped())
    .spawn()
    .expect("Failed to start echo process");

  let echo_out = echo_child.stdout.expect("Failed to open echo stdout");

  let sed_child = Command::new("sed")
    .arg("s/tpyo/typo/")
    .stdin(Stdio::from(echo_out))
    .stdout(Stdio::piped())
    .spawn()
    .expect("Failed to start sed process");

  let output = sed_child.wait_with_output().expect("Failed to wait on sed");
  println!("{:?}", output.stdout);
}
