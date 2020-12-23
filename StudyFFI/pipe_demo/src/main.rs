use libc;
use std::ffi::CString;

fn main() {
  let mut pipefd = [0; 2];
  if unsafe { libc::pipe(&mut pipefd[0]) } == -1 {
    eprintln!("pipe error");
    std::process::exit(0);
  }

  let cpid = unsafe { libc::fork() };
  if cpid == -1 {
    eprintln!("fork error");
    std::process::exit(0);
  }

  if cpid == 0 {
    // child reads from pipe
    unsafe {
      // close unused write end
      libc::close(pipefd[1]);

      let mut buf: libc::c_char = 0;
      while libc::read(
        pipefd[0],
        &mut buf as *mut libc::c_char as *mut libc::c_void,
        1,
      ) > 0
      {
        libc::write(
          libc::STDOUT_FILENO,
          &buf as *const libc::c_char as *const libc::c_void,
          1,
        );
      }

      let cr = CString::new("\n").expect("CString::new failed");
      let cr = cr.as_ptr();
      libc::write(libc::STDOUT_FILENO, cr as *const libc::c_void, 1);

      libc::close(pipefd[0]);

      std::process::exit(0);
    }
  } else {
    // parent writes to pipe
    unsafe {
      // close unused read end
      libc::close(pipefd[0]);

      let hello = CString::new("Hello").expect("CString::new failed");
      let hello = hello.as_ptr();
      libc::write(pipefd[1], hello as *const libc::c_void, libc::strlen(hello));

      libc::close(pipefd[1]); // reader will see EOF

      libc::wait(std::ptr::null_mut()); // wait for child

      std::process::exit(0);
    }
  }
}
