use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;
use std::time::Duration;

/*
因为同时有4个线程在修改 num_done，所有这里没法像例子 ch2_02_progress_reporting
一样使用 store，改为使用 fetch_add。使用 store 可能出现重复赋值的情况。
*/
fn main() {
    let num_done = &AtomicUsize::new(0);
    thread::scope(|s| {
        // Four background threads to process all 100 items, 25 each.
        for t in 0..4 {
            s.spawn(move || {
                for i in 0..25 {
                    process_item(t * 25 + i);
                    num_done.fetch_add(1, Relaxed); 
                }
            });
        }

        // The main thread shows status updates, every second.
        loop {
            let n = num_done.load(Relaxed);
            if n == 100 {
                break;
            }
            println!("Working:.. {n}/100 done");
            thread::sleep(Duration::from_secs(1));
        }
    });
    println!("Done");
}

fn process_item(_: usize) {
    thread::sleep(Duration::from_millis(123));
}
