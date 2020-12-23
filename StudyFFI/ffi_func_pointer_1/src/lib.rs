pub struct Opaque {
  read_callback: Option<unsafe extern "C" fn(ctx: *mut Opaque, data: i32)>,
}

#[no_mangle]
pub extern "C" fn create() -> *mut Opaque {
  Box::into_raw(Box::new(Opaque {
    read_callback: None,
  }))
}

#[no_mangle]
pub extern "C" fn on_read(
  ctx: *mut Opaque,
  read_callback: Option<unsafe extern "C" fn(ctx: *mut Opaque, data: i32)>,
) {
  assert!(!ctx.is_null(), "Invalid parameter: ctx is null.");
  let ctx = unsafe { &mut *ctx };
  ctx.read_callback = read_callback;
}

#[no_mangle]
pub extern "C" fn enter_event_loop(ctx: *mut Opaque) {
  assert!(!ctx.is_null(), "Invalid parameter: ctx is null.");
  let rctx = unsafe { &mut *ctx };
  let read_callback = rctx.read_callback.unwrap();
  for i in 0..10000 {
    unsafe {
      read_callback(ctx, i);
    }
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
