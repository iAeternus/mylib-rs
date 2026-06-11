use std::{
    panic::{self, AssertUnwindSafe},
    sync::{
        Arc, Mutex,
        mpsc::{self, Receiver, SyncSender},
    },
    thread::{self, JoinHandle},
};

use crate::thread_pool::{
    builder::ThreadPoolBuilder,
    stats::{PoolStats, SharedStats},
};

type Task = Box<dyn FnOnce() + Send + 'static>;

#[derive(Debug)]
pub enum ThreadPoolError {
    Closed,
    QueueFull,
}

pub struct TaskHandle<T> {
    receiver: Receiver<T>,
}

impl<T> TaskHandle<T> {
    pub fn join(self) -> Result<T, mpsc::RecvError> {
        self.receiver.recv()
    }
}

pub struct ThreadPool {
    sender: Option<SyncSender<Task>>,
    workers: Vec<JoinHandle<()>>,
    stats: Arc<SharedStats>,
}

impl ThreadPool {
    pub fn new(count: usize) -> Self {
        ThreadPoolBuilder::default().thread_count(count).build()
    }

    pub fn builder() -> ThreadPoolBuilder {
        ThreadPoolBuilder::default()
    }

    pub(crate) fn with_builder(builder: ThreadPoolBuilder) -> Self {
        let (sender, receiver) = mpsc::sync_channel::<Task>(builder.queue_capacity);
        let receiver = Arc::new(Mutex::new(receiver));
        let stats = Arc::new(SharedStats::new());
        let mut workers = Vec::with_capacity(builder.thread_count);

        for id in 0..builder.thread_count {
            let rx = Arc::clone(&receiver);
            let stats = Arc::clone(&stats);

            let handle = thread::Builder::new()
                .name(format!("{}-{}", builder.thread_name, id))
                .spawn(move || worker_loop(rx, stats))
                .expect("failed to spawn worker");

            workers.push(handle);
        }

        Self {
            sender: Some(sender),
            workers,
            stats,
        }
    }

    /// 不关心返回值
    pub fn exec<F>(&self, task: F) -> Result<(), ThreadPoolError>
    where
        F: FnOnce() + Send + 'static,
    {
        let sender = self.sender.as_ref().ok_or(ThreadPoolError::Closed)?;
        self.stats.task_submitted();
        sender
            .send(Box::new(task))
            .map_err(|_| ThreadPoolError::Closed)
    }

    pub fn try_exec<F>(&self, task: F) -> Result<(), ThreadPoolError>
    where
        F: FnOnce() + Send + 'static,
    {
        let sender = self.sender.as_ref().ok_or(ThreadPoolError::Closed)?;
        self.stats.task_submitted();
        sender.try_send(Box::new(task)).map_err(|e| match e {
            mpsc::TrySendError::Full(_) => ThreadPoolError::QueueFull,
            mpsc::TrySendError::Disconnected(_) => ThreadPoolError::Closed,
        })
    }

    /// 获取返回值
    pub fn spawn<F, R>(&self, f: F) -> Result<TaskHandle<R>, ThreadPoolError>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        let sender = self.sender.as_ref().ok_or(ThreadPoolError::Closed)?;
        self.stats.task_submitted();
        let (tx, rx) = mpsc::sync_channel(1);

        let task = Box::new(move || {
            let result = f();
            let _ = tx.send(result);
        });

        sender.send(task).map_err(|_| ThreadPoolError::Closed)?;
        Ok(TaskHandle { receiver: rx })
    }

    pub fn stats(&self) -> PoolStats {
        self.stats.snapshot()
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.sender.take();

        for worker in self.workers.drain(..) {
            let _ = worker.join();
        }
    }
}

fn worker_loop(receiver: Arc<Mutex<Receiver<Task>>>, stats: Arc<SharedStats>) {
    loop {
        let task = {
            let guard = match receiver.lock() {
                Ok(g) => g,
                Err(poisoned) => poisoned.into_inner(),
            };

            guard.recv()
        };

        match task {
            Ok(task) => {
                stats.task_started();

                let result = panic::catch_unwind(AssertUnwindSafe(task));
                if let Err(err) = result {
                    eprintln!("task panic: {:?}", err);
                }

                stats.task_completed();
            }
            Err(_) => break,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), ThreadPoolError> {
        let pool = ThreadPool::builder()
            .thread_count(8)
            .queue_capacity(5000)
            .thread_name("test-worker")
            .build();

        let handle = pool.spawn(|| 100 + 200)?;

        assert_eq!(handle.join().unwrap(), 300);

        Ok(())
    }
}
