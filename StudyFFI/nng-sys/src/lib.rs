#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use anyhow::{anyhow, Context, Result};
use std::ffi::CStr;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn check_error(result: i32) -> Result<()> {
  unsafe {
    if result != 0 {
      let err_str = nng_strerror(result);
      let err_str = CStr::from_ptr(err_str).to_str().unwrap();
      let err_str = String::from(err_str);
      return Err(anyhow!(err_str));
    }
    Ok(())
  }
}

pub fn rnng_pub0_open(socket: *mut nng_socket) -> Result<()> {
  unsafe {
    let result = nng_pub0_open(socket);
    check_error(result).context("nng_pub0_open failed")
  }
}

pub fn rnng_sub0_open(socket: *mut nng_socket) -> Result<()> {
  unsafe {
    let result = nng_sub0_open(socket);
    check_error(result).context("nng_sub0_open failed")
  }
}

pub fn rnng_listen(
  arg1: nng_socket,
  arg2: *const ::std::os::raw::c_char,
  arg3: *mut nng_listener,
  arg4: ::std::os::raw::c_int,
) -> Result<()> {
  unsafe {
    let result = nng_listen(arg1, arg2, arg3, arg4);
    check_error(result).context("nng_listen failed")
  }
}

pub fn rnng_dial(
  arg1: nng_socket,
  arg2: *const ::std::os::raw::c_char,
  arg3: *mut nng_dialer,
  arg4: ::std::os::raw::c_int,
) -> Result<()> {
  unsafe {
    let result = nng_dial(arg1, arg2, arg3, arg4);
    check_error(result).context("nng_dial failed")
  }
}

pub fn rnng_setopt(
  arg1: nng_socket,
  arg2: *const ::std::os::raw::c_char,
  arg3: *const ::std::os::raw::c_void,
  arg4: size_t,
) -> Result<()> {
  unsafe {
    let result = nng_setopt(arg1, arg2, arg3, arg4);
    check_error(result).context("nng_setopt failed")
  }
}

pub fn rnng_msg_alloc(arg1: *mut *mut nng_msg, arg2: size_t) -> Result<()> {
  unsafe {
    let result = nng_msg_alloc(arg1, arg2);
    check_error(result).context("nng_setopt failed")
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
