use futures::future;
use futures::select;

fn main() {
  println!("Hello, world!");

  let mut a_fut = future::ready(4);
  let mut b_fut = future::ready(6);
  let mut c_fut = future::pending::<()>();
  let mut total = 0;

  loop {
    select! {
        a = a_fut => total += a,
        b = b_fut => total += b,
        c = c_fut => { println!("c"); },
        //complete => break,
        default => panic!(),
    };
  }
  println!("total = {}", total);
}
