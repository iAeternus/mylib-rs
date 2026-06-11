use std::sync::{
    Mutex,
    atomic::{AtomicUsize, Ordering},
};

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

    pub(super) fn register(&self, thread: std::thread::Thread) {
        self.threads.lock().unwrap().push(thread);
    }

    pub(super) fn park(&self) {
        self.sleeping.fetch_add(1, Ordering::SeqCst);
        std::thread::park();
        self.sleeping.fetch_sub(1, Ordering::SeqCst);
    }

    pub(super) fn unpark_one(&self) {
        let threads = self.threads.lock().unwrap();
        if let Some(t) = threads.first() {
            t.unpark();
        }
    }

    pub(super) fn unpark_all(&self) {
        let threads = self.threads.lock().unwrap();
        for t in threads.iter() {
            t.unpark();
        }
    }
}
