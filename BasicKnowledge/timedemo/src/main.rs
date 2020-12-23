use std::time::{Duration, SystemTime};

fn main() {
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
    println!("Hello, world! {:?}", now);
}
