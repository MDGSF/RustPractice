use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
  Ok(cx.string("hello node"))
}

fn print_function(mut cx: FunctionContext) -> JsResult<JsFunction> {
  let arg0 = cx.argument::<JsFunction>(0)?;
  Ok(arg0)
}

fn add_1_to_argument(mut cx: FunctionContext) -> JsResult<JsNumber> {
  let x = cx.argument::<JsNumber>(0)?.value();
  Ok(cx.number(x + 1.0))
}

fn get_args_len(mut cx: FunctionContext) -> JsResult<JsNumber> {
  let args_length = cx.len();
  Ok(cx.number(args_length))
}

fn args_opt(mut cx: FunctionContext) -> JsResult<JsUndefined> {
  match cx.argument_opt(0) {
    Some(arg) => {
      let num = arg.downcast::<JsNumber>().or_throw(&mut cx)?.value();
      println!("The 0th argument is {}", num);
    }
    None => panic!("0th argument does not exist, out of bounds!"),
  }
  Ok(cx.undefined())
}

fn default_args(mut cx: FunctionContext) -> JsResult<JsUndefined> {
  let age = match cx.argument_opt(0) {
    Some(arg) => arg.downcast::<JsNumber>().or_throw(&mut cx)?.value(),
    None => 18 as f64,
  };

  let name = match cx.argument_opt(1) {
    Some(arg) => arg.downcast::<JsString>().or_throw(&mut cx)?.value(),
    None => "Huang Jian".to_string(),
  };

  println!("I am {} years old and my name is {}", age, name);

  Ok(cx.undefined())
}

fn accepts_js_arrays(mut cx: FunctionContext) -> JsResult<JsNumber> {
  let js_arr_handle: Handle<JsArray> = cx.argument(0)?;
  let vec: Vec<Handle<JsValue>> = js_arr_handle.to_vec(&mut cx)?;
  let vec_of_numbers: Vec<f64> = vec
    .iter()
    .map(|js_value| {
      js_value
        .downcast::<JsNumber>()
        .unwrap_or(cx.number(0))
        .value()
    })
    .collect();
  let sum: f64 = vec_of_numbers.iter().sum();
  Ok(cx.number(sum))
}

fn accepts_js_objects(mut cx: FunctionContext) -> JsResult<JsString> {
  let js_object_handle: Handle<JsObject> = cx.argument(0)?;
  let js_object = js_object_handle
    .downcast::<JsObject>()
    .unwrap_or(JsObject::new(&mut cx));
  let rust_string = js_object
    .get(&mut cx, "myProp")?
    .downcast::<JsString>()
    .unwrap_or(cx.string(""));
  Ok(cx.string(rust_string.value()))
}

register_module!(mut cx, {
  cx.export_function("hello", hello)?;
  cx.export_function("printFunction", print_function)?;
  cx.export_function("add1ToArgument", add_1_to_argument)?;
  cx.export_function("getArgsLen", get_args_len)?;
  cx.export_function("argsOpt", args_opt)?;
  cx.export_function("defaultArgs", default_args)?;
  cx.export_function("acceptsJsArrays", accepts_js_arrays)?;
  cx.export_function("acceptsJsObjects", accepts_js_objects)?;
  Ok(())
});
