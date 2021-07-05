use threadpool::ThreadPool;

fn main() {
  let pool = ThreadPool::new(3);

  for i in 1..=5 {
    pool.execute(move || {
      println!("number {} from spawned_1 thread!", i);
    });
  }

  for i in 1..=5 {
    pool.execute(move || {
      println!("number {} from spawned_2 thread!", i);
    });
  }

  for i in 1..=5 {
    println!("number {} from main thread!", i);
  }

  pool.join();
}
