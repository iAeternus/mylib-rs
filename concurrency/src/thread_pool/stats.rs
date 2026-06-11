use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone, Copy)]
pub struct PoolStats {
    pub submitted: usize,
    pub running: usize,
    pub completed: usize,
}

pub(crate) struct SharedStats {
    submitted: AtomicUsize,
    running: AtomicUsize,
    completed: AtomicUsize,
}

impl SharedStats {
    pub(crate) fn new() -> Self {
        Self {
            submitted: AtomicUsize::new(0),
            running: AtomicUsize::new(0),
            completed: AtomicUsize::new(0),
        }
    }

    pub(crate) fn task_submitted(&self) {
        self.submitted.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn task_started(&self) {
        self.running.fetch_add(1, Ordering::Relaxed);
    }

    pub(crate) fn task_completed(&self) {
        self.running.fetch_sub(1, Ordering::Relaxed);
        self.completed.fetch_add(1, Ordering::Relaxed);
    }

    pub fn snapshot(&self) -> PoolStats {
        PoolStats {
            submitted: self.submitted.load(Ordering::Relaxed),
            running: self.running.load(Ordering::Relaxed),
            completed: self.completed.load(Ordering::Relaxed),
        }
    }
}
