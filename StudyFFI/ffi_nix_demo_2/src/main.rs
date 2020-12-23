use nix::sys::signal::*;
use nix::unistd::*;

fn main() {
  match fork().expect("fork failed") {
    ForkResult::Parent { child } => {
      sleep(5);
      kill(child, SIGKILL).expect("kill failed");
    }
    ForkResult::Child => loop {},
  }
}
