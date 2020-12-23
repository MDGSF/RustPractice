use libc;
use std::ffi::CString;
use std::io;

fn main() {
  let mut pipe1 = [0; 2];
  let mut pipe2 = [0; 2];

  unsafe {
    libc::pipe(&mut pipe1[0]);
    libc::pipe(&mut pipe2[0]);
  }

  let childpid = unsafe { libc::fork() };
  if childpid == 0 {
    // child
    unsafe {
      libc::close(pipe1[1]);
      libc::close(pipe2[0]);
    }
    server(pipe1[0], pipe2[1]);
    std::process::exit(0);
  }

  // parent
  unsafe {
    libc::close(pipe1[0]);
    libc::close(pipe2[1]);
  }

  client(pipe2[0], pipe1[1]);

  unsafe {
    libc::waitpid(childpid, std::ptr::null_mut(), 0);
  }
  std::process::exit(0);
}

const MAXLINE: usize = 4096;

fn server(readfd: libc::c_int, writefd: libc::c_int) {
  unsafe {
    let mut buf: [libc::c_char; MAXLINE + 1] = [0; MAXLINE + 1];
    let n = libc::read(
      readfd,
      &mut buf[0] as *mut libc::c_char as *mut libc::c_void,
      MAXLINE,
    );
    if n == 0 {
      println!("end-of-file while reading pathname");
      std::process::exit(0);
    }
    let n = n as usize;
    buf[n] = b'\0' as i8;

    let fd = libc::open(&buf[0], libc::O_RDONLY);
    if fd < 0 {
      libc::snprintf(
        &mut buf[n],
        MAXLINE + 1 - n,
        b": can't open, %s\n" as *const u8 as *const i8,
        libc::strerror(*libc::__errno_location()),
      );

      libc::write(
        writefd,
        &buf[0] as *const libc::c_char as *const libc::c_void,
        libc::strlen(&buf[0]),
      );
    } else {
      loop {
        let n = libc::read(
          fd,
          &mut buf[0] as *mut libc::c_char as *mut libc::c_void,
          MAXLINE,
        );
        if n <= 0 {
          break;
        }

        libc::write(
          writefd,
          &buf[0] as *const libc::c_char as *const libc::c_void,
          n as usize,
        );
      }
      libc::close(fd);
    }
  }
}

fn client(readfd: libc::c_int, writefd: libc::c_int) {
  let mut filename = String::new();
  io::stdin()
    .read_line(&mut filename)
    .expect("failed to read line");

  let rfilename = filename.trim();

  let cfilename = CString::new(rfilename.clone()).unwrap();
  let cfilename = cfilename.as_ptr();

  unsafe {
    libc::write(writefd, cfilename as *const libc::c_void, rfilename.len());

    let mut buf: [libc::c_char; MAXLINE] = [0; MAXLINE];
    loop {
      let n = libc::read(
        readfd,
        &mut buf[0] as *mut libc::c_char as *mut libc::c_void,
        MAXLINE,
      );
      if n <= 0 {
        break;
      }

      libc::write(
        libc::STDOUT_FILENO,
        &buf[0] as *const libc::c_char as *const libc::c_void,
        n as usize,
      );
    }
  }
}
