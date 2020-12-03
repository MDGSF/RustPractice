use neon::prelude::*;
use num_cpus;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
  Ok(cx.string("hello node"))
}

fn thread_count(mut cx: FunctionContext) -> JsResult<JsNumber> {
  Ok(cx.number(num_cpus::get() as f64))
}

register_module!(mut cx, {
  cx.export_function("hello", hello)?;
  cx.export_function("threadCount", thread_count)?;
  Ok(())
});
