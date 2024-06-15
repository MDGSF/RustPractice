/*
特性：
1. 支持多生产者、多消费者。

缺点：
1. 多个生产者和消费者公用一个 mutex，效率可能比较低
2. VecDeque 触发扩容的时候，导致其他人都被 block 住，效率可能没法接受
3. 发送者的速率如果大于消费者，会导致队列无限增大
*/
use std::collections::VecDeque;
use std::sync::Condvar;
use std::sync::Mutex;

pub struct Channel<T> {
    queue: Mutex<VecDeque<T>>,
    item_ready: Condvar,
}

impl<T> Channel<T> {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
            item_ready: Condvar::new(),
        }
    }

    pub fn send(&self, message: T) {
        self.queue.lock().unwrap().push_back(message);
        self.item_ready.notify_one();
    }

    pub fn receive(&self) -> T {
        let mut b = self.queue.lock().unwrap();
        loop {
            if let Some(message) = b.pop_front() {
                return message;
            }
            b = self.item_ready.wait(b).unwrap();
        }
    }
}
