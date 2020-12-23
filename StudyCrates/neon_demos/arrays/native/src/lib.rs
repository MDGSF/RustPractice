use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
  Ok(cx.string("hello node"))
}

fn convert_vec_to_array(mut cx: FunctionContext) -> JsResult<JsArray> {
  let vec: Vec<String> = (0..10).map(|num| num.to_string()).collect();

  let js_array = JsArray::new(&mut cx, vec.len() as u32);

  vec.iter().enumerate().for_each(|e| {
    let (i, obj) = e;
    let js_string = cx.string(obj);
    let _ = js_array.set(&mut cx, i as u32, js_string);
  });

  Ok(js_array)
}

fn convert_js_array_to_vec(mut cx: FunctionContext) -> JsResult<JsNumber> {
  let js_arr_handle: Handle<JsArray> = cx.argument(0)?;
  let vec: Vec<Handle<JsValue>> = js_arr_handle.to_vec(&mut cx)?;
  Ok(cx.number(vec.len() as f64))
}

fn return_empty_js_array(mut cx: FunctionContext) -> JsResult<JsArray> {
  Ok(cx.empty_array())
}

fn return_js_array_with_number(mut cx: FunctionContext) -> JsResult<JsArray> {
  let array: Handle<JsArray> = JsArray::new(&mut cx, 1);
  let n = cx.number(9000.0);
  array.set(&mut cx, 0, n)?;
  Ok(array)
}

fn return_js_array_with_string(mut cx: FunctionContext) -> JsResult<JsArray> {
  let array: Handle<JsArray> = JsArray::new(&mut cx, 1);
  let s = cx.string("hello node");
  array.set(&mut cx, 0, s)?;
  Ok(array)
}

register_module!(mut cx, {
  cx.export_function("hello", hello)?;
  cx.export_function("convertVecToArray", convert_vec_to_array)?;
  cx.export_function("convertJsArrayToVec", convert_js_array_to_vec)?;
  cx.export_function("returnJsArray", return_empty_js_array)?;
  cx.export_function("returnJsArrayWithNumber", return_js_array_with_number)?;
  cx.export_function("returnJsArrayWithString", return_js_array_with_string)?;
  Ok(())
});
