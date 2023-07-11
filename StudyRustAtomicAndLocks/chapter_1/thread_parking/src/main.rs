use std::collections::VecDeque;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

/*
An important property of thread parking is that a call to unpark() before the thread parks itself does not get lost.

However, unpark requests don’t stack up. Calling unpark() two times and then calling park() two times afterwards still results in the thread going to sleep. The first park() clears the request and returns directly, but the second one goes to sleep as usual.

当有多个消费者时，park 和 unpark 就不好使了。
*/
fn main() {
    let queue = Mutex::new(VecDeque::new());
    thread::scope(|s| {
        // Comsuming thread
        let t = s.spawn(|| loop {
            let item = queue.lock().unwrap().pop_front();
            if let Some(item) = item {
                dbg!(item);
            } else {
                // 这个有点坑，在没有调用 unpark() 的情况下，也有可能会唤醒该线程。
                // 所以需要确保，把 park 和 unpark 去掉之后，代码逻辑仍然是正确的。
                thread::park();
            }
        });

        // Producing thread
        for i in 0.. {
            queue.lock().unwrap().push_back(i);
            t.thread().unpark();
            thread::sleep(Duration::from_secs(1));
        }
    });
}
