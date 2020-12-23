use std::env::args;
use std::process::*;

fn main() {
  let mut arg_iter = args();
  arg_iter.next();
  let pattern = arg_iter.next().unwrap_or("main".to_string());
  let pt = arg_iter.next().unwrap_or("./".to_string());
  let child = Command::new("grep")
    .arg("-n")
    .arg("-r")
    .arg(&pattern)
    .arg(&pt)
    .stdout(Stdio::piped())
    .spawn()
    .unwrap();

  // std::thread::sleep_ms(1000);
  let out = child.wait_with_output().unwrap();
  let out_str = String::from_utf8_lossy(&out.stdout);
  for line in out_str.split("\n") {
    println!("{}", line);
  }
}
