use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
  Ok(cx.string("hello node"))
}

fn binary_js_to_rust(mut cx: FunctionContext) -> JsResult<JsNumber> {
  let b: Handle<JsArrayBuffer> = cx.argument(0)?;
  let slice = cx.borrow(&b, |data| {
    let slice = data.as_slice::<u8>();
    slice
  });
  println!("{:#?}", slice);
  Ok(cx.number(slice.len() as f64))
}

fn binary_rust_to_js(mut cx: FunctionContext) -> JsResult<JsArrayBuffer> {
  let mut buffer = JsArrayBuffer::new(&mut cx, 8)?;
  let slice = cx.borrow_mut(&mut buffer, |data| {
    let slice = data.as_mut_slice::<u8>();
    slice
  });
  for i in 0..8 {
    slice[i] = i as u8;
  }
  Ok(buffer)
}

register_module!(mut cx, {
  cx.export_function("hello", hello)?;
  cx.export_function("binaryJsToRust", binary_js_to_rust)?;
  cx.export_function("binaryRustToJs", binary_rust_to_js)?;
  Ok(())
});
