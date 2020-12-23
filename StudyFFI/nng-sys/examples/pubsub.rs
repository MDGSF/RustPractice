/// https://nanomsg.org/gettingstarted/nng/pubsub.html
/// ./pubsub server ipc:///tmp/pubsub.ipc
/// ./pubsub client ipc:///tmp/pubsub.ipc
/// 
/// ./pubsub server tcp://127.0.0.1:8080
/// ./pubsub client tcp://127.0.0.1:8080

use anyhow::{anyhow, Result};
use nng_sys::*;
use std::env;
use std::{
  ffi::{CStr, CString},
  os::raw::c_char,
  os::raw::c_void,
  ptr::null_mut,
};

fn server(url: &str) -> Result<()> {
  unsafe {
    let curl = CString::new(url).unwrap();
    let curl = curl.as_bytes_with_nul().as_ptr() as *const c_char;

    let mut socket = nng_socket::default();
    rnng_pub0_open(&mut socket)?;
    rnng_listen(socket, curl, null_mut(), 0)?;

    println!("server start listen at: {}", url);

    loop {
      let mut msg: *mut nng_msg = null_mut();
      rnng_msg_alloc(&mut msg, 0)?;

      let val = 0x12345678;
      nng_msg_append_u32(msg, val);

      nng_sendmsg(socket, msg, 0);

      std::thread::sleep(std::time::Duration::from_secs(1));
    }
  }
}

fn client(url: &str) -> Result<()> {
  unsafe {
    let curl = CString::new(url).unwrap();
    let curl = curl.as_bytes_with_nul().as_ptr() as *const c_char;

    let opt = CStr::from_bytes_with_nul(NNG_OPT_SUB_SUBSCRIBE).unwrap();
    let opt = opt.as_ptr() as *const c_char;

    let val = CString::new("").unwrap();
    let val = val.as_bytes_with_nul().as_ptr() as *const c_void;

    let mut socket = nng_socket::default();

    rnng_sub0_open(&mut socket)?;

    rnng_setopt(socket, opt, val, 0)?;

    rnng_dial(socket, curl, null_mut(), 0)?;

    println!("client connect to {} success", url);

    loop {
      let mut msg: *mut nng_msg = null_mut();
      nng_recvmsg(socket, &mut msg, 0);

      let mut recv_val: u32 = 0;
      nng_msg_trim_u32(msg, &mut recv_val);

      println!("recv_val = {}", recv_val);
    }
  }
}

fn main() -> Result<()> {
  let args: Vec<String> = env::args().collect();
  if args.len() <= 1 {
    return Err(anyhow!("Usage: pubsub [client|server] [url]"));
  }
  match args[1].as_str() {
    "client" => client(args[2].as_str()),
    "server" => server(args[2].as_str()),
    _ => Err(anyhow!("Usage: pubsub [client|server] [url]")),
  }
}
