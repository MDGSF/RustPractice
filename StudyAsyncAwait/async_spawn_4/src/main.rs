use async_std::task;

fn main() {
  task::spawn_blocking(|| myloop());

  std::thread::sleep(std::time::Duration::from_secs(1000));
}

fn myloop() {
  for i in 0.. {
    println!("myloop i = {}", i);
    std::thread::sleep(std::time::Duration::from_secs(1));
  }
}
