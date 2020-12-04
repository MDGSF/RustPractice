use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
  Ok(cx.string("hello node"))
}

fn throw_error(mut cx: FunctionContext) -> JsResult<JsString> {
  let arg0 = cx.argument::<JsString>(0)?.value();

  if !arg0.contains("hello") {
    // Equaivalent to `throw new Error('Expected you to say hello')` in JS
    panic!("Expected you so say hello");
  }

  Ok(cx.string("hello to you too!"))
}

fn throw_type_error(mut cx: FunctionContext) -> JsResult<JsValue> {
  let _foo: JsResult<JsError> = cx.throw_type_error("not enough arguments")?;
  Ok(cx.string("throw_error node").upcast())
}

fn throw_if_string_not_includes_foo(mut cx: FunctionContext) -> JsResult<JsValue> {
  let arg0 = cx.argument::<JsString>(0)?.value();
  if !arg0.contains("foo") {
    return cx.throw_error("The given string does not contain 'foo'");
  }
  Ok(cx.string("The given string has 'foo'!").upcast())
}

fn throw_custom_error(mut cx: FunctionContext) -> JsResult<JsError> {
  let arg0 = cx.argument::<JsString>(0)?.value();
  match arg0.as_str() {
    "type_error" => cx.throw_type_error("throwing a TypeError"),
    "range_error" => cx.throw_range_error("throwing a RangeError"),
    "error" => cx.throw_error("throwing an Error"),
    _ => cx.throw_error("please pass an expected error type"),
  }
}

fn create_error_obj(mut cx: FunctionContext) -> JsResult<JsError> {
  let arg0 = cx.argument::<JsString>(0)?.value();
  match arg0.as_str() {
    "type_error" => cx.type_error("throwing a TypeError"),
    "range_error" => cx.range_error("throwing a RangeError"),
    "error" => cx.error("throwing an Error"),
    _ => cx.throw_error("please pass an expected error type"),
  }
}

register_module!(mut cx, {
  cx.export_function("hello", hello)?;
  cx.export_function("throwError", throw_error)?;
  cx.export_function(
    "throwIfStringNotIncludesFoo",
    throw_if_string_not_includes_foo,
  )?;
  cx.export_function("throwCustomError", throw_custom_error)?;
  cx.export_function("throwTypeError", throw_type_error)?;
  cx.export_function("createErrorObj", create_error_obj)?;
  Ok(())
});
