use std::sync::atomic::AtomicU64;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;
use std::time::Duration;
use std::time::Instant;

/*
下面代码存在一些问题：
1. 可能出现 num_done 更新了，然后 total_time 还没有更新，这个时候 main thread 
统计出的结果就不对。
2. 因为 Relaxed 不保证 memory order，所以可能出现获取到新的 total_time，
但是却获取到旧的 num_done。

要解决这些问题，可以把3个原子变量全部放到一个 Mutex 中，不过这样效率更低。
*/
fn main() {
    let num_done = &AtomicUsize::new(0);
    let total_time = &AtomicU64::new(0);
    let max_time = &AtomicU64::new(0);

    thread::scope(|s| {
        // Four background threads to process all 100 items, 25 each.
        for t in 0..4 {
            s.spawn(move || {
                for i in 0..25 {
                    let start = Instant::now();
                    process_item(t * 25 + i);
                    let time_taken = start.elapsed().as_micros() as u64;
                    num_done.fetch_add(1, Relaxed);
                    total_time.fetch_add(time_taken, Relaxed);
                    max_time.fetch_max(time_taken, Relaxed);
                }
            });
        }

        // The main thread shows status updates, every second.
        loop {
            let total_time = Duration::from_micros(total_time.load(Relaxed));
            let max_time = Duration::from_micros(max_time.load(Relaxed));
            let n = num_done.load(Relaxed);
            if n == 100 {
                break;
            } else if n == 0 {
                println!("Working:.. nothing done yet.");
            } else {
                println!(
                    "Working:.. {n}/100 done, {:?} average, {:?} peak",
                    total_time / n as u32,
                    max_time
                );
            }
            thread::sleep(Duration::from_secs(1));
        }
    });
    println!("Done");
}

fn process_item(_: usize) {
    thread::sleep(Duration::from_millis(123));
}
