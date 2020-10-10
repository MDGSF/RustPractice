use nix::unistd::*;

fn main() {
  match fork() {
    Ok(ForkResult::Parent { child: _ }) => {
      println!("Hello, I am parent thread: {}", getpid());
    }
    Ok(ForkResult::Child) => {
      println!("Hello, I am child thread: {}", getpid());
      println!("My parent thread: {}", getppid());
    }
    Err(errno) => {
      println!("Fork creation failed!, errno = {}", errno);
    }
  }
}
