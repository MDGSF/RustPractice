use {
    futures::{
        future::{BoxFuture, FutureExt},
        task::{waker_ref, ArcWake},
    },
    std::{
        future::Future,
        sync::mpsc::{sync_channel, Receiver, SyncSender},
        sync::{Arc, Mutex},
        task::{Context, Poll},
        time::Duration,
    },
    timer_future::TimerFuture,
};

/// 任务执行器，负责从通道中接收任务然后执行
struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

/// `Spawner` 负责创建新的 future，然后将它发送到任务通道中
#[derive(Clone)]
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

/// 一个future，它可以将自己放入任务通道中，然后等待执行器去`poll`
struct Task {
    /// 进行中的future
    future: Mutex<Option<BoxFuture<'static, ()>>>,

    /// 可以将该任务自己放回到任务通道中，等待执行器的`poll`
    task_sender: SyncSender<Arc<Task>>,
}

fn new_executor_and_spawner() -> (Executor, Spawner) {
    const MAX_QUEUE_TASKS: usize = 10000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUE_TASKS);
    (Executor { ready_queue }, Spawner { task_sender })
}

impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("任务队列已满");
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self.task_sender.send(cloned).expect("任务队列已满");
    }
}

impl Executor {
    fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&*waker);
                if future.as_mut().poll(context).is_pending() {
                    *future_slot = Some(future);
                }
            }
        }
    }
}

fn main() {
    let (executor, spawner) = new_executor_and_spawner();

    spawner.spawn(async {
        println!("howdy!");
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("done!");
    });

    drop(spawner);

    executor.run();
}
