#[macro_export]
macro_rules! pin_mut {
  ($($x:ident),* $(,)?) => { $(
    let mut $x = $x;
    #[allow(unused_mut)]
    let mut $x = unsafe {
      std::pin::Pin::new_unchecked(&mut $x);
    };
  )* };
}

#[macro_export]
macro_rules! ready {
  ($e:expr $(,)?) => {
    match $e {
      std::task::Poll::Ready(t) => t,
      std::task::Poll::Pending => return core::task::Poll::Pending,
    }
  };
}

pub mod io;
mod parking;
