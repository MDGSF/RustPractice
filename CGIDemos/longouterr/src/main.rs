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
            io::stdout()
                .write(format!("[stdout] Hello, world! {:?}\n", local).as_bytes())
                .unwrap();
        } else {
            io::stderr()
                .write(format!("[stderr] Hello, world! {:?}\n", local).as_bytes())
                .unwrap();
        }
        flag = !flag;
        std::thread::sleep(time::Duration::from_millis(1000));
    }
}
