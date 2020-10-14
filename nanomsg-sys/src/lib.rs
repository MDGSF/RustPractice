#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use anyhow::{anyhow, Result};
use std::ffi::CStr;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn rnn_errno() -> ::std::os::raw::c_int {
  unsafe { nn_errno() }
}

pub fn rnn_strerror(errnum: ::std::os::raw::c_int) -> String {
  unsafe {
    let result = nn_strerror(errnum);
    CStr::from_ptr(result).to_string_lossy().into_owned()
  }
}

pub fn rnn_socket(
  domain: ::std::os::raw::c_int,
  protocol: ::std::os::raw::c_int,
) -> Result<::std::os::raw::c_int> {
  unsafe {
    let socket = nn_socket(domain, protocol);
    if socket < 0 {
      return Err(anyhow!("nn_socket: {:?}", rnn_strerror(rnn_errno())));
    }
    Ok(socket)
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
