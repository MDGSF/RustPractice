use std::time;
use chrono::prelude::*;

fn main() {
    loop {
        let local: DateTime<Local> = Local::now();
        println!("[stdout] Hello, world! {:?}", local);
        std::thread::sleep(time::Duration::from_millis(1000));
    }
}
