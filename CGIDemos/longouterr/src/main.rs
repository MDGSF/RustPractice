use chrono::prelude::*;
use std::io;
use std::io::prelude::*;
use std::thread;
use std::time;

fn main() {
    thread::spawn(move || loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        io::stdout().write(buffer.as_bytes()).unwrap();
    });

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
