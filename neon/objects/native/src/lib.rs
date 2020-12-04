use neon::prelude::*;

struct Foo {
  pub bar: u64,
  pub baz: String,
  age: u32,
}

impl Foo {
  pub fn new(bar: u64, baz: String) -> Foo {
    Foo { bar, baz, age: 123 }
  }

  pub fn test_publibc(&self) {
    println!("test_publibc");
  }

  fn test_private(&self) {
    println!("test_private");
  }
}

fn convert_struct_to_js_object(mut cx: FunctionContext) -> JsResult<JsObject> {
  let foo = Foo::new(1234, "baz".to_string());
  let object = JsObject::new(&mut cx);
  let js_string = cx.string(&foo.baz);
  let js_number = cx.number(foo.bar as f64);
  object.set(&mut cx, "myStringProperty", js_string).unwrap();
  object.set(&mut cx, "myNumberProperty", js_number).unwrap();
  Ok(object)
}

register_module!(mut cx, {
  cx.export_function("convertStructToJsObject", convert_struct_to_js_object)?;
  Ok(())
});
