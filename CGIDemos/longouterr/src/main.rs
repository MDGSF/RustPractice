use chrono::prelude::*;
use std::time;

fn main() {
    let mut flag = true;
    loop {
        let local: DateTime<Local> = Local::now();
        if flag {
            println!("[stdout] Hello, world! {:?}", local);
        } else {
            eprintln!("[stderr] Hello, world! {:?}", local);
        }
        flag = !flag;
        std::thread::sleep(time::Duration::from_millis(1000));
    }
}
