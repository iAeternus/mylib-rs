use std::sync::{
    Mutex,
    atomic::{AtomicUsize, Ordering},
};

/// 基于 `thread::park/unpark` 的 Worker 睡眠/唤醒（内建令牌，不会丢失唤醒信号）。
pub(crate) struct Sleeper {
    sleeping: AtomicUsize,
    threads: Mutex<Vec<std::thread::Thread>>,
}

impl Sleeper {
    pub(super) fn new() -> Self {
        Self {
            sleeping: AtomicUsize::new(0),
            threads: Mutex::new(Vec::new()),
        }
    }

    /// 将 Worker 线程注册到唤醒列表。
    pub(super) fn register(&self, thread: std::thread::Thread) {
        self.threads.lock().unwrap().push(thread);
    }

    /// 当前线程睡眠直到被 `unpark_one/all` 唤醒。
    pub(super) fn park(&self) {
        self.sleeping.fetch_add(1, Ordering::SeqCst);
        std::thread::park();
        self.sleeping.fetch_sub(1, Ordering::SeqCst);
    }

    /// 唤醒一个 Worker。
    pub(super) fn unpark_one(&self) {
        let threads = self.threads.lock().unwrap();
        if let Some(t) = threads.first() {
            t.unpark();
        }
    }

    /// 唤醒所有 Worker。
    pub(super) fn unpark_all(&self) {
        let threads = self.threads.lock().unwrap();
        for t in threads.iter() {
            t.unpark();
        }
    }
}
