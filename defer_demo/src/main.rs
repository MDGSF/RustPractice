macro_rules! defer {
    ($($body:tt)*) => {
        let _guard = {
            pub struct Guard<F: FnOnce()>(Option<F>);

            impl<F: FnOnce()> Drop for Guard<F> {
                fn drop(&mut self) {
                    (self.0).take().map(|f| f());
                }
            }

            Guard(Some(|| {
                let _ = { $($body)* };
            }))
        };
    };
}

fn main() {
  defer! {
    println!("hello defer");
    for i in 0..10 {
      println!("defer i = {}", i);
    }
    println!("defer end");
  }

  println!("Hello, world!");
}
