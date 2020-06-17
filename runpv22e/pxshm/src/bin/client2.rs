use libc;
use pxshm::*;
use runpv22e::*;
use std::ffi::CString;
use std::{mem, ptr};
use std::{thread, time};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "client2")]
struct Opt {
  #[structopt(name = "name")]
  name: String,

  #[structopt(name = "loops")]
  loops: usize,

  #[structopt(name = "usec")]
  usec: u64,
}

fn main() {
  let opt = Opt::from_args();

  let fd = rshm_open(&opt.name, libc::O_RDWR, FILE_MODE);

  let ptr = rmmap(
    ptr::null_mut(),
    mem::size_of::<shmstruct>(),
    libc::PROT_READ | libc::PROT_WRITE,
    libc::MAP_SHARED,
    fd,
    0,
  );
  let ptr = ptr as *mut shmstruct;
  let ptr = unsafe { &mut *ptr };

  rclose(fd);

  println!("before loop, opt = {:?}", opt);

  let pid = std::process::id();
  for i in 0..opt.loops {
    thread::sleep(time::Duration::from_millis(opt.usec));
    let msg = format!("pid {}, message {}", pid, i);

    if rsem_trywait(&mut ptr.nempty) == -1 {
      if unsafe { *libc::__errno_location() } == libc::EAGAIN {
        rsem_wait(&mut ptr.noverflowmutex);
        ptr.noverflow += 1;
        rsem_post(&mut ptr.noverflowmutex);
      } else {
        println!("rsem_trywait error");
        std::process::exit(1);
      }
    }

    rsem_wait(&mut ptr.mutex);
    let offset: usize = ptr.msgoff[ptr.nput as usize] as usize;
    ptr.nput += 1;
    if ptr.nput as usize >= NMESG {
      ptr.nput = 0;
    }
    rsem_post(&mut ptr.mutex);

    let cmsg = CString::new(msg).unwrap();
    let cmsg = cmsg.as_ptr();
    unsafe {
      libc::strcpy(&mut ptr.msgdata[offset], cmsg);
    }
    rsem_post(&mut ptr.nstored);
  }
}
