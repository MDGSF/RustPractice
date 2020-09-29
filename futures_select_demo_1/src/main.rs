use async_std::task;
use futures::future;
use futures::select;

fn main() {
  task::block_on(async {
    let mut a = future::ready(4);
    let mut b = future::pending::<()>();
    let res = select! {
      a_res = a => a_res + 1,
      _ = b => 0,
    };
    println!("res = {}", res);
  });
}
