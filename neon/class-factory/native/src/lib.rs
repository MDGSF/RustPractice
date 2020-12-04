extern crate neon;

use std::ops::Drop;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
  Ok(cx.string("hello node"))
}

pub struct Logger {
  name: String,
  counter: Arc<AtomicU64>,
}

impl Logger {
  pub fn log(&self, s: &str) {
    let count = self.counter.load(Ordering::Relaxed);
    println!("Loggers: {}, {}: {}", count, self.name, s);
  }
}

impl Drop for Logger {
  fn drop(&mut self) {
    self.counter.fetch_sub(1, Ordering::Relaxed);
  }
}

pub struct LazyLogger {
  pub logger: Option<Logger>,
}

pub struct LogFactory {
  counter: Arc<AtomicU64>,
}

impl LogFactory {
  pub fn new() -> Self {
    LogFactory {
      counter: Arc::new(AtomicU64::new(0)),
    }
  }

  pub fn create(&self, name: String) -> Logger {
    self.counter.fetch_add(1, Ordering::Relaxed);
    Logger {
      name,
      counter: self.counter.clone(),
    }
  }
}

declare_types! {
  pub class JsLogger for Logger {
    init(mut cx) {
      let factory = cx.argument::<JsLogFactory>(0)?;
      let name = cx.argument::<JsString>(1)?.value();

      let guard = cx.lock();
      let factory = factory.borrow(&guard);

      Ok(factory.create(name))
    }

    method log(mut cx) {
      let log = cx.argument::<JsString>(0)?;
      let this = cx.this();
      let guard = cx.lock();

      this.borrow(&guard).log(&log.value());

      Ok(cx.undefined().upcast())
    }
  }

  pub class JsLazyLogger for LazyLogger {
    init(_) {
      Ok(LazyLogger {
        logger: None,
      })
    }

    method log(mut cx) {
      let log = cx.argument::<JsString>(0)?;
      let this = cx.this();

      cx.borrow(&this, |logger| {
        let logger = logger.logger.as_ref().unwrap();
        logger.log(&log.value());
      });

      Ok(cx.undefined().upcast())
    }
  }

  pub class JsLogFactory for LogFactory {
    init(_) {
      Ok(LogFactory::new())
    }

    method create(mut cx) {
      let name = cx.argument::<JsValue>(0)?;
      let this = cx.this().upcast();
      Ok(JsLogger::new(&mut cx, vec![this, name])?.upcast())
    }

    method lazy(mut cx) {
      let name = cx.argument::<JsString>(0)?;
      let this = cx.this();
      let inner = cx.borrow(&this, |factory| factory.create(name.value()));

      let mut logger = JsLazyLogger::new::<_, JsLazyLogger, _>(&mut cx, vec![])?;
      let guard = cx.lock();

      logger.borrow_mut(&guard).logger = Some(inner);

      Ok(logger.upcast())
    }
  }
}

register_module!(mut cx, {
  cx.export_function("hello", hello)?;
  cx.export_class::<JsLogFactory>("LogFactory")?;
  Ok(())
});
